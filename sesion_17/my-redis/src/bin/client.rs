use mini_redis::{client, Result};
use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;


type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
	Get {
		key: String,
		resp: Responder<Option<Bytes>>
	},
	Set {
		key: String, 
		val: Bytes,
		resp: Responder<()>
	}
}

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

	let t1 = tokio::spawn(async move{
		//let res = client.get("foo").await;
		let (resp_tx, resp_rx) = oneshot::channel();
		let cmd = Command::Get { 
			key:"foo".to_string(),
			resp: resp_tx,			
		};
		
		// enviar con mpsc comando que tiene oneshot para leer respuesta
		if tx.send(cmd).await.is_err(){
			eprintln!("connection task shutdown");
			return
		};

		let res = resp_rx.await;
		println!("GOT = {:?}", res);
	});


	let t2 = tokio::spawn(async move{
		let (resp_tx, resp_rx) = oneshot::channel();
		let cmd = Command::Set {
			key: "foo".to_string(),
			val: "bar".into(),
			resp: resp_tx,			
		};
		if tx2.send(cmd).await.is_err() {
			eprintln!("connection task shutdown");
			return	
		};
		//client.set("foo", "bar".into()).await;
		let res = resp_rx.await;
		println!("GOT = {:?}", res);
	});


	let manager = tokio::spawn(async move {
		let mut client = client::connect("127.0.0.1:6688").await.unwrap();

		while let Some(cmd) = rx.recv().await {
			println!("GOT = {:?}", cmd);
			use Command::*;
			match cmd {
				Get {key, resp} => {
					let res = client.get(&key).await;
					let _ = resp.send(res);
				},
				Set {key, val, resp} => {					
					let res = client.set(&key, val).await;
					let _ = resp.send(res);
				}
			}

		}
	});

    t1.await.unwrap();
    t2.await.unwrap();
	manager.await.unwrap();

    Ok(())
}
