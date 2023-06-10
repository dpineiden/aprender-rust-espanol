use tonic::{transport::Server, Request, Response, Status};
use prueba_saludo::prueba_protocol::{SaludoRequest, SaludoReply};
use prueba_saludo::prueba_protocol::prueba_service_server::{
	PruebaService,PruebaServiceServer};


#[derive(Default)]
pub struct SaludoService {}

// asociar el PruebaService trait con SaludoService

#[tonic::async_trait]
impl PruebaService for SaludoService {
	async fn saludar(&self, request:Request<SaludoRequest>
	) -> Result<Response<SaludoReply>, Status> {
		let nombre = request.into_inner().nombre;
		let response = format!("Hola {nombre}, ¿cómo estás?");
		let reply = SaludoReply {saludo: response};
		Ok(Response::new(reply))		
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "[::1]:50051".parse().unwrap();
	let service = SaludoService::default();

    Server::builder()
        .add_service(PruebaServiceServer::new(service))
        .serve(addr)
        .await?;


	Ok(())
}
