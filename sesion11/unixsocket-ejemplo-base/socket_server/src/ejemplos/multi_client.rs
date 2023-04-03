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
	let mut buffer = [0; 100];
	loop{
		println!("Entering loop for this client");
		match stream.read(&mut buffer[..]).await{
			Ok(n)=>{
				let txt = str::from_utf8(&buffer[..n]).unwrap(); 
				println!("Received from client {:?}", txt);
			},
			Err(err)=>{
				stream.shutdown(Shutdown::Both).unwrap();
				break;
			}

		};	

		task::sleep(Duration::from_secs(1)).await;

		if &received == "END" {
			println!("Cerrando servicio a client");
			break}
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
