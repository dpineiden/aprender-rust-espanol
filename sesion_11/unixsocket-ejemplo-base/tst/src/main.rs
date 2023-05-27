use async_std::prelude::*;
use async_std::os::unix::net::UnixStream;
use std::net::Shutdown;
use async_std::io;
use std::time::Duration;

#[async_std::main]
async fn main() {

	let socket_path = "/tmp/rust.socket";
	let mut stream = UnixStream::connect(socket_path).await.unwrap(); 
	let address = stream.local_addr().unwrap();
	println!("Local Unix Socket:: {:?}", address);
	let peer_address = stream.peer_addr().unwrap();
	println!("Remote Unix Socket:: {:?}", peer_address);
	// lectura del stream


	let stdin = io::stdin();
	let mut line = String::new();
	match stdin.read_line(&mut line).await {
		Ok(size)=>{println!("Recibido Input {}", size)},
		Err(err) => println!("Error")
	};
	println!("Enviado a server : {}", line);
}
