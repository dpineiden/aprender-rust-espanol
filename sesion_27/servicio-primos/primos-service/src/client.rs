use tonic::{transport::Server, Request, Response, Status};
use primos_service::primos_protocol::{PrimosRequest, PrimosResponse};
use primos_service::primos_protocol::primos_service_client::PrimosServiceClient;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use primos::primeros_primos as gen_primos;
use tokio_stream::StreamExt;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


	let start = Instant::now();
	let mut client = PrimosServiceClient::connect("http://[::1]:50051").await?;
	let request = Request::new(PrimosRequest{n:100000});
	let response = client.primeros_primos(request).await?;
	let mut stream = response.into_inner();
	while let Some(msg) = stream.message().await? {
		println!("Response MSG = {:?}", msg.primo);
	}
	let duration_1 = start.elapsed();

	let start = Instant::now();
	println!("Llamando async-primeros-primos");
	let request = Request::new(PrimosRequest{n:100000});
	let response = client.async_primeros_primos(request).await?;
	let mut stream = response.into_inner();
	while let Some(msg) = stream.message().await? {
		println!("Response MSG = {:?}", msg.primo);
	}
	let duration_2 = start.elapsed();


	println!("Duracion 1 {:?}", duration_1);
	println!("Duracion 2 {:?}", duration_2);

	println!("Proporcion {}", (duration_2.subsec_nanos() as
	f32)/(duration_1.subsec_nanos()) as f32);


	Ok(())
}
