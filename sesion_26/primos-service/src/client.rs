use tonic::{transport::Server, Request, Response, Status};
use primos_service::primos_protocol::{PrimosRequest, PrimosResponse};
use primos_service::primos_protocol::primos_service_client::PrimosServiceClient;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use primos::primeros_primos as gen_primos;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	let mut client = PrimosServiceClient::connect("http://[::1]:50051").await?;
	let request = Request::new(PrimosRequest{n:100});
	let response = client.primeros_primos(request).await?;
	let mut stream = response.into_inner();
	while let Some(msg) = stream.message().await? {
		println!("Response MSG = {:?}", msg.primo);
	}


	Ok(())
}
