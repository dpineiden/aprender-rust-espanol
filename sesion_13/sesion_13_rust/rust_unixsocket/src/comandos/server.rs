use async_std::{fs,task};
use std::path::Path;
use async_std::os::unix::net::UnixListener;
use async_std::prelude::*;
use rust_unixsocket::handler::handle_client;


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
