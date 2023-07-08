use tonic::transport::Server;//, Request, Response, Status};
use std::path::Path;
use grpc_device_status::service::status::DeviceService;
use grpc_device_status::rpc_status_device::status_device_service_server::{StatusDeviceServiceServer};



#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
	let mut service = DeviceService::default().await;

	service.add(1, "test_1", Path::new("/home/david/Documentos"));
	service.add(2, "test_2", Path::new("/media/david/ExtraPineidenX/Rust_Books/"));


	tokio::spawn(async move {
		let addr = "127.0.0.1:50001".parse().unwrap();
		Server::builder().add_service(
			StatusDeviceServiceServer::new(service)
		).serve(addr).await.unwrap();
	}).await.unwrap();

	Ok(())
}
