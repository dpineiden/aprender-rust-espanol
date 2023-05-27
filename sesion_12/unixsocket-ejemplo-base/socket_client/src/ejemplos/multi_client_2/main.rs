use async_std::prelude::*;
use async_std::os::unix::net::UnixStream;
use std::net::Shutdown;
use async_std::io;
mod read_from_stream;
use read_from_stream::read_stream;

#[async_std::main]
async fn main() {
	let socket_path = "/tmp/rust.socket";
	let mut stream = UnixStream::connect(socket_path).await.unwrap(); 
	let address = stream.local_addr().unwrap();
	println!("Local Unix Socket:: {:?}", address);
	let peer_address = stream.peer_addr().unwrap();
	println!("Remote Unix Socket:: {:?}", peer_address);
	// lectura del stream
	//let mut response = String::new();
	//stream.read_to_string(&mut response).await.unwrap();
	//println!("Recibido desde servidor: {}", response);
	// close
	// loop interactivo

	let stdin = io::stdin();
	let mut line = String::new();
	const size:usize = 10;
	let stop = b'\0';

	loop{
		let msg = read_stream(
			&mut stream, 
			size, 
			stop).await;
		println!("Received from server {:?}", msg);

		match stdin.read_line(&mut line).await {
			Err(err) => println!("Error"),
			Ok(n)=>println!("Received from server {} bytes", n)
		};
		println!("Enviado a server : {}", line);

		match stream.write(line.as_bytes()).await{
			Ok(s)=>println!("Enviados {} bytes", s),
			Err(err)=>eprintln!("Error al enviar")
		};
		stream.flush().await.unwrap();
		if &line=="END"{
			println!("Cerrando cliente");
			break
		}
		line.clear();
		}
	stream.shutdown(Shutdown::Both).unwrap();
}
