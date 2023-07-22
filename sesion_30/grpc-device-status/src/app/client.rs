use grpc_device_status::rpc_status_device::status_device_service_client::{StatusDeviceServiceClient};


use grpc_device_status::rpc_status_device::{DeviceRequest, DeviceCreateRequest};


use std::time::Instant;
use futures_util::StreamExt;
use tonic::Request;
use std::io::{self, BufRead};
use inquire::{error::InquireError,Select};
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum CRUDAction {
	READ,
	CREATE,
	UPDATE,
	DELETE,
	LIST
}

impl fmt::Display for CRUDAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for CRUDAction {

    type Err = ();

    fn from_str(input: &str) -> Result<CRUDAction, Self::Err> {
        match input.to_ascii_uppercase().as_str() {
            "READ"  => Ok(CRUDAction::READ),
            "CREATE"  => Ok(CRUDAction::CREATE),
            "UPDATE"  => Ok(CRUDAction::CREATE),
            "DELETE"  => Ok(CRUDAction::CREATE),
            "LIST"  => Ok(CRUDAction::LIST),
            _      => Ok(CRUDAction::READ),
        }
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "http://127.0.0.1:50001";

	let mut client = StatusDeviceServiceClient::connect(addr).await;


    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();

	let options = vec!["READ", "CREATE", "LIST"];

	/* inquire interaction */
	let ans: Result<&str, InquireError> = Select::new("¿Qué quieres hacer?", options).prompt();

	let mut control = CRUDAction::READ;
	match ans {
		Ok(choice) => {
			println!("Elegiste hacer: {}!", choice);
			control = CRUDAction::from_str(choice).unwrap();
		},
		Err(_) => println!("Hubo un error, intenta de nuevo"),
	}
	/* end inquire interaction */
	println!("Control is {}", control);
	match control {
		CRUDAction::CREATE => {
			while let Some(line) = lines.next() {
				/* obtener nombre */
				
				let name = line.unwrap();

				// stop reading
				if name.len() == 0 {
					break;
				}

				let line = lines.next().unwrap();
				/* obtener path en string */
				println!("Path line {:?}", line);
				let path = line.unwrap();

				// stop reading
				if path.len() == 0 {
					break;
				}

				/* crear DeviceCreateRequest { name,  path}*/

				let request = Request::new(DeviceCreateRequest{name, path});
				match client.as_mut().expect("No value").create_device_register(request).await{
					Ok(response)=>{
						println!("Reponse Unique ===> MSG = {:?}", response.into_inner());
					},
					Err(_)=>
					{
						println!("NO hay respuesta para esta acción de
				crear");
						continue
					}
                }
			}
		},
		CRUDAction::READ => {
			while let Some(line) = lines.next() {
				let last_input = line.unwrap();

				// stop reading
				if last_input.len() == 0 {
					break;
				}

				// add a new line once user_input starts storing user input
				if user_input.len() > 0 {
					user_input.push_str("\n");
				}

				// store user input
				let input_value: u32 = match last_input.trim().parse() {
						Ok(num) => num,
						Err(_) => continue,
					};

				let request = Request::new(DeviceRequest{ids:input_value});
				match client.as_mut().expect("No
				value").get_memory_info(request).await{
				Ok(response)=>{
						println!("Reponse Unique ===> MSG = {:?}", response.into_inner());
				 },
				Err(_)=>
				{
				println!("NO hay respuesta para este id {}", input_value);
				continue
				}
				};
			}

		},
		_ => {
			println!("Por hacer");
			todo!()
		}

	}

	Ok(())
}
