use async_std::prelude::*;
use async_std::os::unix::net::{UnixListener, UnixStream};
use std::time::Duration;
use std::net::Shutdown;
use async_std::task;
use async_std::fs;
use std::str;
use std::path::Path;
mod read;
use read::read_stream;


async fn handle_client(mut stream: UnixStream) {
	/*
	Tres tareas principales: 

	- enviar mensajes
	- procesar mensajes
	- recibir mensajes
	 */

	println!("Nuevo cliente {:?}", stream.peer_addr().unwrap());
	stream.write(b"Hola mundo socket").await.unwrap();
	stream.flush().await.unwrap();

	let mut buffer:[u8;100] = [0;100];

	const size:usize= 10;
	let stop = b'\0';
	println!("Stop value {}", stop);


	loop {
		println!("Entering loop for this client");
		let mut txt = read_stream(
			&mut stream, size, stop).await;

		println!("Recibido desde cliente {:?}, empty {}", 
				 txt, 
				 txt.is_empty());

		if txt.is_empty() &&  &txt == "END"{
			println!("Cerrando servicio a client");
			stream.shutdown(Shutdown::Both).unwrap();
			break;
		};

		task::sleep(Duration::from_secs(1)).await;

		let txt_up = txt.to_uppercase();
		let msg = format!("Recibido en server: {txt_up}");

		match stream.write(msg.as_bytes()).await {
			Ok(n)=>{},
			Err(err)=>{
				stream.shutdown(Shutdown::Both).unwrap();
			}
		};
		stream.flush().await.unwrap();
	}

}


#[async_std::main]
async fn main() {
	let socket_path = "/tmp/rust.socket";
	let path = Path::new(socket_path);
	if path.exists() {
		fs::remove_file(path).await.expect("El archivo no pudo borrarse");
	}
	let listener = UnixListener::bind(socket_path).await.unwrap();
	let mut incoming = listener.incoming();
	
	while let Some(stream) = incoming.next().await {
		match stream {
			Ok(stream) => {
				task::spawn(handle_client(stream));
				//stream.write_all(b"Hola mundo socket").await.unwrap();
			},
			Err(err) => {
				eprintln!("Error al conectar cliente socket {:?}", err)
			}
		}

	}

}

