use tonic::{transport::Server, Request, Response, Status};
use primos_service::primos_protocol::{PrimosRequest, PrimosResponse};
use primos_service::primos_protocol::primos_service_server::{
	PrimosService as PrimosServiceRPC,PrimosServiceServer};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use primos::primeros_primos as gen_primos;
use primos::primeros_primos_stream as async_gen_primos;

//use tokio_stream::StreamExt;
use futures_core::Stream;
use futures_util::StreamExt;

#[derive(Default)]
pub struct PrimosService {}

#[tonic::async_trait]
impl PrimosServiceRPC for PrimosService {
	type PrimerosPrimosStream=ReceiverStream<Result<PrimosResponse, Status>>;
	type AsyncPrimerosPrimosStream=ReceiverStream<Result<PrimosResponse, Status>>;

	async fn primeros_primos(
		&self, 
		request:Request<PrimosRequest>) ->
		Result<Response<Self::PrimerosPrimosStream>,Status> {

		let (tx, rx) = mpsc::channel(1200);
		
		tokio::spawn(async move {
			let msg = request.into_inner();
			let vec_primos = gen_primos(msg.n);
			for primo in vec_primos.into_iter() {
				let primo_response = PrimosResponse {primo:primo};
				tx.send(Ok(primo_response)).await.unwrap();
			}
		});
		
		Ok(Response::new(ReceiverStream::new(rx)))
	}



	async fn async_primeros_primos(
		&self, 
		request:Request<PrimosRequest>) ->
		Result<Response<Self::PrimerosPrimosStream>,Status> {

		let (tx_u64, mut rx_u64) = mpsc::channel(120);
		let (tx, rx) = mpsc::channel(120);
		
		tokio::spawn(async move {
			let msg = request.into_inner();
			let vec_primos = async_gen_primos(msg.n, &tx_u64).await;
		});

		tokio::spawn(async move {
			while let Some(i) = rx_u64.recv().await {
				if i==0 {break; }
				let primo_response = PrimosResponse {primo:i};
				tx.send(Ok(primo_response)).await.unwrap();
			}
		});
		
		Ok(Response::new(ReceiverStream::new(rx)))
	}


}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "[::1]:50051".parse().unwrap();
	let service = PrimosService::default();

    Server::builder()
        .add_service(PrimosServiceServer::new(service))
        .serve(addr)
        .await?;


	Ok(())
}
