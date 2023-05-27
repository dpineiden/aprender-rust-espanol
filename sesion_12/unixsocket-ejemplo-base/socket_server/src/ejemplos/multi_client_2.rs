use async_std::prelude::*;
use async_std::os::unix::net::{UnixListener,UnixStream};
use std::path::Path;
use async_std::fs;
use async_std::task;
use std::time::Duration;
use std::net::Shutdown;
use std::str;



async fn handle_client(mut stream:UnixStream) {
	/*
	tres tareas principales: 

	- enviar mensajes
	- procesar mensajes
	- recibir mensajes

	 */
	println!("Nuevo cliente: {:?}", stream.peer_addr().unwrap());

	// envio
	stream.write(b"Hola mundo socket").await.unwrap();
	stream.flush().await.unwrap();

	let mut received = String::new();
	const size:usize = 10;
	let stop = b'\0';
	println!("Stop value {}", stop);
	
	let mut txt = String::new();

	loop{
		println!("Entering loop for this client");
		let mut buff_vector:Vec<[u8;size]> = vec![];

		loop{
			let mut buffer = [0; size];

			match stream.read(&mut buffer[..]).await{
				Ok(n)=>{
					buff_vector.push(buffer);
					if buffer.iter().any(|&x|x==stop){
						break;
					}
				},
				Err(err)=>{
					stream.shutdown(Shutdown::Both).unwrap();
					break;
				}

			};	
			//buffer = [0;size];
			
		}

		let allchars:Vec<u8> = buff_vector
			.concat()
			.iter()
			.filter_map(|&x|if x!=stop {Some(x)} else {None})
			.collect();

		txt = str::from_utf8(&allchars).unwrap().trim().to_string(); 
		println!("Received from client {:?}, empty: {}", txt, txt.is_empty());
		if txt.is_empty() {
			stream.shutdown(Shutdown::Both).unwrap();
			break;
		}
		task::sleep(Duration::from_secs(1)).await;

		if &received == "END" {
			println!("Cerrando servicio a client");
			break}
		
		// enviar respuesta de servidor a cliente
		let text_up = txt.to_uppercase();
		let msg = format!("Recibido en server: {text_up}");
		
		match stream.write(msg.as_bytes()).await{
			Ok(n)=>{},
			Err(err)=>stream.shutdown(Shutdown::Both).unwrap()
		};
		stream.flush().await.unwrap();
	}
}

#[async_std::main]
async fn main() {
	let socket_path = "/tmp/rust.socket";
	let path = Path::new(socket_path);
	if path.exists() {
		fs::remove_file(path).await.expect("Borrado de socket falló");
	}
	
	let listener = UnixListener::bind(socket_path).await.unwrap(); 
	let mut incoming = listener.incoming();

	while let Some(stream) = incoming.next().await {
		match stream {
			Ok(stream) => {
				task::spawn(handle_client(stream));
			},
			Err(err) => {
				eprintln!("Error al conectar socket {:?}",err);
			}
		}
	}
}
