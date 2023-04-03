use async_std::prelude::*;
use async_std::os::unix::net::{UnixListener,UnixStream};
use std::path::Path;
use async_std::fs;
use async_std::task;
use std::time::Duration;
use std::net::Shutdown;

mod read;
use read::read_stream;

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

	const size:usize = 10;
	let stop = b'\0';
	println!("Stop value {}", stop);
	
	let mut txt = String::new();

	loop{
		println!("Entering loop for this client");
		let mut txt = read_stream(&mut stream, size, stop).await;

		println!("Received from client {:?}, empty: {}", txt, txt.is_empty());
		if txt.is_empty() {
			stream.shutdown(Shutdown::Both).unwrap();
			break;
		}
		task::sleep(Duration::from_secs(1)).await;

		if &txt == "END" {
			println!("Cerrando servicio a client");
			stream.shutdown(Shutdown::Both).unwrap();
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
