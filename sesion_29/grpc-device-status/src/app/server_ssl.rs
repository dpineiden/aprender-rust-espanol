use tonic::transport::Server;//, Request, Response, Status};
use std::path::Path;
use grpc_device_status::service::status::DeviceService;
use grpc_device_status::rpc_status_device::status_device_service_server::{StatusDeviceServiceServer};
use tokio_rustls::{
    rustls::{Certificate, PrivateKey, ServerConfig},
    TlsAcceptor,
};
use tokio::net::TcpListener;


#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "ssl"]);
    let certs = {
        let fd = std::fs::File::open(data_dir.join("public.key"))?;
        let mut buf = std::io::BufReader::new(&fd);
        rustls_pemfile::certs(&mut buf)?
            .into_iter()
            .map(Certificate)
            .collect()
    };

	let key = {
        let fd = std::fs::File::open(data_dir.join("safe.cert"))?;
        let mut buf = std::io::BufReader::new(&fd);
        rustls_pemfile::pkcs8_private_keys(&mut buf)?
            .into_iter()
            .map(PrivateKey)
            .next()
            .unwrap()
	};	

    let mut tls = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;
    tls.alpn_protocols = vec![b"h2".to_vec()];



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
