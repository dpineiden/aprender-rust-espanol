use tonic::{transport::Server, Request, Response, Status};
use prueba_saludo::prueba_protocol::{SaludoRequest, SaludoReply};
use prueba_saludo::prueba_protocol::prueba_service_client::PruebaServiceClient;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut client = PruebaServiceClient::connect("http://[::1]:50051").await?;
	let request =
		Request::new(SaludoRequest{nombre:"Juanita".into()});
	let response = client.saludar(request).await?;
	println!("RESPONSE={:?}", response);

	Ok(())
}
