use async_std::prelude::*;
use async_std::os::unix::net::UnixStream;
use std::net::Shutdown;
use std::str;
use crate::mensaje::Message;

pub async fn read_stream(
	stream:&mut UnixStream, 
	size:usize, 
	stop:u8) -> Message
{
		let mut buff_vector:Vec<Vec<u8>> = vec![];
		let mut flag = true;
		let mut head:u32 = 0;
	    let mut counter:u32=0;

		loop{
			let mut buffer = vec![0; size];	
			match stream.read(&mut buffer[..]).await{
				Ok(n)=>{
					// cantidad de bytes a leer
					if flag {
						let head_bytes:[u8;4] = [
							buffer[0], 
							buffer[1],
							buffer[2],
							buffer[3]];
						
						head = u32::from_ne_bytes(head_bytes);
						flag = false;
					}

					counter += buffer.len() as u32;
					println!("buffer: {:?}, {:?}",&buffer, &stop);
					if counter >= head + 4 {
						buff_vector.push(buffer);
						break;
					} else {
						buff_vector.push(buffer);
					}
				},
				Err(err)=>{
					stream.shutdown(Shutdown::Both).unwrap();
					break;
				}

			};	
		}

		let allchars:Vec<u8> = buff_vector
			.concat()
			.iter()
			.filter_map(|&x|if x!=stop {Some(x)} else {None})
			.collect();

		//str::from_utf8(&allchars).unwrap().to_string()
	Message::deserialize(&allchars)
}
