use tonic::transport::{Server, Identity,ServerTlsConfig};
use tonic::transport::server::{TcpConnectInfo,TlsConnectInfo};

use std::fs::File;
use csv::ReaderBuilder;
use std::io;
use std::error::Error;

use grpc_device_status::service::status::DeviceService;
use grpc_device_status::rpc_status_device::status_device_service_server::{StatusDeviceServiceServer};
use clap; // 3.1.6
use clap::Parser;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};




/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
	#[arg(long)]
    host: Ipv4Addr,

    /// Number of times to greet
    #[arg(long)]
    port: u16,

    #[arg(short, long)]
	settings: PathBuf
}

impl Args {
	pub fn address(&self)->String{
		format!("{}:{}",self.host, self.port)
	} 
}

#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct CsvData {
    id: u32,
    name: String,
    path: PathBuf,
}



fn read_csv(service:&mut DeviceService, file_path: &PathBuf)  ->
    Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
	let mut iter = rdr.deserialize();
    for result in iter {
        let record:CsvData = result?;
		println!("Dato: {:?}", record);

        let id: u32 = record.id;
        let name = record.name.as_str();
        let path = record.path;

        // Llama al método "add" del servicio para agregar los datos
        service.add(id, name, &path);
    }
		Ok(())
}


#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	println!("{:?}", args);
	println!("Address  {:?}", args.address());

	// futuro: args: path ssl
    let data_dir = std::path::PathBuf::from_iter([std::env!("CARGO_MANIFEST_DIR"), "ssl"]);
	let cert = std::fs::read_to_string(data_dir.join("rootCA.crt"))?;
	let key = std::fs::read_to_string(data_dir.join("key.pem"))?;

	let identity = Identity::from_pem(cert, key);

	let mut service = DeviceService::default().await;
	read_csv(&mut service, &args.settings)?;

	tokio::spawn(async move {
		let addr = args.address().parse().unwrap();
		Server::builder()
			.tls_config(ServerTlsConfig::new().identity(identity)).expect("Correct
		tls certificates")
			.add_service(StatusDeviceServiceServer::new(service)).serve(addr).await.unwrap();
	}).await.unwrap();

	Ok(())
}
