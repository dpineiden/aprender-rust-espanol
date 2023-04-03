use async_std::prelude::*;
use async_std::os::unix::net::UnixStream;
use std::net::Shutdown;


#[async_std::main]
async fn main() {
	let socket_path = "/tmp/rust.socket";
	let mut stream = UnixStream::connect(socket_path).await.unwrap(); 
	let address = stream.local_addr().unwrap();
	println!("Local Unix Socket:: {:?}", address);
	let peer_address = stream.peer_addr().unwrap();
	println!("Remote Unix Socket:: {:?}", peer_address);
	// lectura del stream
	let mut response = String::new();
	stream.read_to_string(&mut response).await.unwrap();
	println!("Recibido desde servidor: {}", response);
	// close
	stream.shutdown(Shutdown::Both).unwrap();
}
