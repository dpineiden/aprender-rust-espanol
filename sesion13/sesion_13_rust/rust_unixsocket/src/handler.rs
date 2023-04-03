use async_std::prelude::*;
use async_std::os::unix::net::UnixStream;
use async_std::task;
use std::time::Duration;
use std::net::Shutdown;

use crate::read::read_stream;
use crate::mensaje::Message;

use arrayref::array_ref;

pub async fn handle_client(mut stream:UnixStream) {
	/*
	tres tareas principales: 

	- enviar mensajes
	- procesar mensajes
	- recibir mensajes

	 */
	let msg = Message::welcome(10);
	println!("Welcome msg {:?}", msg);
	println!("Welcome msg bytes {:?}", msg.serialize());

	println!("Nuevo cliente: {:?}", stream.peer_addr().unwrap());
	let stop = b'\n';
	let msg_bytes  = msg.serialize();

	println!("Total largo msg {}", msg_bytes.len());
	// envio
	stream.write(&msg_bytes).await.unwrap();
	stream.write(&[stop]).await.unwrap();
	stream.flush().await.unwrap();


	let msg_recovered = Message::deserialize(&msg_bytes);
	println!("Welcome msg recuperado {:?}", &msg_recovered);

	const size:usize = 10;
	println!("Stop value {}", stop);
	
	let mut txt = String::new();

	loop{
		println!("Entering loop for this client");
		let mut txt = read_stream(&mut stream, size, stop).await;

		task::sleep(Duration::from_secs(1)).await;

		
		// enviar respuesta de servidor a cliente
		
		match stream.write(&msg.serialize()).await{
			Ok(n)=>{},
			Err(err)=>stream.shutdown(Shutdown::Both).unwrap()
		};
		stream.flush().await.unwrap();
	}
}
