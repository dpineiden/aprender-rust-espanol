use grpc_device_status::rpc_status_device::status_device_service_client::{StatusDeviceServiceClient};


use grpc_device_status::rpc_status_device::{DeviceRequest};


use std::time::Instant;
use futures_util::StreamExt;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "http://127.0.0.1:50001";
	let client = StatusDeviceServiceClient::connect(addr).await;

	let request = Request::new(DeviceRequest{ids:1});
	let response_unique = client.expect("No value").get_memory_info(request).await?;
	println!("Reponse Unique ===> MSG = {:?}", response_unique);
	Ok(())
}
