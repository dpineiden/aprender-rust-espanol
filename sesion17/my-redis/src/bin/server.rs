use tokio::net::{TcpListener,TcpStream};
use mini_redis::{Connection, Frame};
use std::sync::{Arc, Mutex, MutexGuard};
use bytes::Bytes;
use std::collections::HashMap;
// Arc -> Rc

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

// type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

// struct CanIncrement {
// 	mutex: Mutex<i32>
// }

// impl CanIncrement {
// 	fn increment(&self){
// 		let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
// 		*lock += 1;
// 	}
// }


// fn new_sharded_db(num_shards:usize) -> ShardedDb { 
// 	let mut db = Vec::with_capacity(num_shards);
// 	for _ in 0..num_shards {
// 		db.push(Mutex::new(HashMap::new()));
// 	}
// 	Arc::new(db)
// }

// async fn increment_and_do_stuff(can_incre:&CanIncrement){
// 	can_incr.increment();
// 	do_something_async().await;
// }


#[tokio::main]
async fn main()  {
	let listener = TcpListener::bind("127.0.0.1:6688").await.unwrap();

	let db = Arc::new(Mutex::new(HashMap::new()));

	loop {
		let (socket, _) = listener.accept().await.unwrap();
		let db = db.clone();
		tokio::spawn( async move {
			process(socket, db).await;
		});
	}
}

async fn process(socket:TcpStream, db:Db) {
	use mini_redis::Command::{self, Get, Set};
	use std::collections::HashMap;

	// aqui vamos a almacenar los valores
	//let mut db = HashMap::new();

	let mut connection = Connection::new(socket);

	
	while let Some(frame) = connection.read_frame().await.unwrap() {
		let response = match Command::from_frame(frame).unwrap() {
			Set(cmd)=>{
				db.insert(cmd.key().to_string(), cmd.value().to_vec());
				Frame::Simple("OK".to_string())
			},
			Get(cmd)=>{
				if let Some(value) = db.get(cmd.key()) {
					Frame::Bulk(value.clone().into())
				} else {
					Frame::Null
				}
			},
			cmd => panic!("unimplemented {:?}", cmd)
		};
		connection.write_frame(&response).await.unwrap();
	}
	// if let Some(frame) = connection.read_frame().await.unwrap() {
	// 	println!("GOT {:?}", frame);
	// 	let response =Frame::Error("uninplemented".to_string());
	// 	connection.write_frame(&response).await.unwrap();		
	// }

}
