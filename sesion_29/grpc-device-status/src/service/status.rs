use std::path::{Path, PathBuf};
use tonic::async_trait;

// crate específico que necesitamos
use device_status::info::{Device, MemoryHumanInfo, Memory};

// mensajes para componer acciones
use crate::rpc_status_device::{
	Memory as rpcM, MemoryHumanInfo as rpcMHF,
	DeviceReply, DeviceRequest,
	DeviceDeletedReply,
	DeviceUpdateReply, DeviceUpdateRequest, 
	DeviceCreateReply, DeviceCreateRequest
};


use crate::rpc_status_device::status_device_service_server::{StatusDeviceService};

use tonic::{Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use tonic::Code;
use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use std::env;
use dotenv::dotenv;
use chrono::prelude::*;

#[derive(Debug,Clone)]
pub struct DeviceItem {
	id: u32,
	name: String,
	db_path: PathBuf
}

impl DeviceItem {
	pub fn get_id(&self)->u32{
		self.id
	}

	pub fn get_name(&self)->String {
		self.name.clone()
	}
	
	pub fn get_db_path(&self)-> PathBuf {
		self.db_path.clone()
	}


	pub fn new(id:u32, name:&str, db_path:&Path) -> Self {
		Self {id, name:name.into(), db_path:db_path.into()}
	}

	pub fn get_measure(&self)-> DeviceReply {
		let device = Device::new(self.id, &self.name, &self.db_path);
		device.into()//convertir Device -> DeviceReply
	}
	
}
use prost_types::Timestamp;

impl From<Memory> for rpcM {
	fn from(value:Memory) -> Self {
		Self {
			available:value.get_available(),
			free: value.get_free(),
			total: value.get_total()		
		}
	}
}

impl From<MemoryHumanInfo> for rpcMHF {
	fn from(value:MemoryHumanInfo) -> Self {
		Self {
			available:value.get_available(),
			percentage: value.get_percentage(),
			scala: value.get_scala() as i32		
		}
	}

}


/*Tarea:: revisar como implementar for para traits de otros crates*/
// impl From<DateTime<Utc>> for Timestamp {
// 	fn from(dt:DateTime<Utc>) -> Self {
// 		let seconds = dt.timestamp() as i64;
// 		let nanos = dt.timestamp_subsec_nanos() as i32;
// 		Self {seconds, nanos};
// 	}
// }

impl From<Device> for DeviceReply {
	fn from(value:Device)->Self {
		// convertira Timestamp
		let dt = value.get_datetime();
		let seconds = dt.timestamp() as i64;
		let nanos = dt.timestamp_subsec_nanos() as i32;
		let ts:Timestamp = Timestamp {seconds, nanos};
		Self{
			ids: value.get_id(),
			datetime:Some(ts),
			name: value.get_name(),
			host: value.get_host(),
			path: value.get_db_path().into_os_string().into_string().unwrap(),
			memory: Some(value.get_memory().into()),
			memory_human: Some(value.memory_info().into())
		}

		}

}


#[derive(Clone,Debug)]
pub struct DeviceService {
	items: Arc<Mutex<HashMap<u32, DeviceItem>>>
}



impl DeviceService {
	pub async fn default()-> Self {
		let mutex_map = Mutex::new(HashMap::new());
		let arc_map = Arc::new(mutex_map);
		dotenv().ok();
		// db_url -> futuro
		Self {items:arc_map}
	}

	//void
	pub fn add(&mut self, id: u32, name:&str, db_path: &Path) {
		let mut items = self.items.lock().unwrap();
		items.insert(id, DeviceItem::new(id, name, db_path));
	}
	
	pub fn get(&self, id:&u32)->Option<DeviceItem> {
		let mut items = self.items.lock().unwrap();
		items.get(id).cloned()
	}

	pub fn new_id(&self)-> u32{
		let items = self.items.lock().unwrap();
		let ids:u32 = items.values().fold(u32::MIN, |acc, item_b|
										  acc.max(item_b.get_id()));
		ids + 1
	}

	pub fn create(&self,
				  msg:&DeviceCreateRequest
	)->Option<DeviceCreateReply>{
		let newid = self.new_id();
		let path = Path::new(&msg.path);
		// habilitar modificación	
		let item = DeviceItem::new(newid, &msg.name, path);
		let mut items = self.items.lock().unwrap();
		items.insert(newid, item);
		Some(DeviceCreateReply {ids:newid, created:true})
	}

}


/*
StatusDeviceServiceServer -> DeviceService
*/


#[async_trait]
impl StatusDeviceService for DeviceService{
	async fn get_memory_info(
		&self,
		request:Request<DeviceRequest>
	)->Result<Response<DeviceReply>,Status>
	{
		let msg = request.into_inner();
		match self.get(&msg.ids){
			Some(reply)=>Ok(Response::new(reply.get_measure())),
			None => Err(Status::new(Code::InvalidArgument, "Ids is invalid"))
		}
	}

	async fn create_device_register(
		&self,
		request:Request<DeviceCreateRequest>) ->
		Result<Response<DeviceCreateReply>, Status> {
			// que se modifica?
			let msg = request.into_inner();
			match self.create(&msg){
				Some(reply)=>{
					Ok(Response::new(reply))
				},
				None => Err(Status::new(Code::InvalidArgument, "Ids cannot be created"))
			}
		}

}
