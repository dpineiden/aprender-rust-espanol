use std::thread;
use std::time::Duration;
use std::sync::mpsc;


#[derive(Debug)]
struct Identificador {
	pub name: String,
	pub valor: i32
}

impl Identificador {
	pub fn new(name: &str, valor:i32) -> Self {
		Self{name:name.to_owned(), valor}
	}
}


fn main() {

	// crear canal: tx: Sender<T>, rx: Receiver<T>
	let (tx, rx) =  mpsc::channel::<Identificador>();
	let tx_2 = tx.clone();


	let builder = thread::Builder::new();
	let builder2 = thread::Builder::new();
	

	let handle = builder.name("Hilo Iteracion >=0".to_string()).spawn(move || {
		let name = thread::current().name().unwrap().to_string();
		for i in 1..10 {	
			let new_id = Identificador::new(&name, i);
			tx.send(new_id).unwrap();
			thread::sleep(Duration::from_millis(200));
		}
		// END CODE
		println!("Sending END signal");
		let end_id = Identificador::new("END", 0);
		tx.send(end_id).unwrap();
	}).unwrap();
	

	let handle = builder2.name("Hilo Iteracion <=0".to_string()).spawn(move || {
		let name = thread::current().name().unwrap().to_string();
		for i in 1..10 {	
			let new_id = Identificador::new(&name, -i);
			tx_2.send(new_id).unwrap();
			thread::sleep(Duration::from_millis(112));
		}
		// END CODE
		println!("Sending END signal");
		let end_id = Identificador::new("END", 0);
		tx_2.send(end_id).unwrap();
	}).unwrap();


	let mut counter=0;

	loop {
		let v = rx.recv().unwrap();
		println!("Hola estoy en main, recibo {:?}", v);
		if (v.name=="END"){
			if (counter<2){counter+=1;} 
			if (counter==2){break}
		}
		thread::sleep(Duration::from_millis(100));
	}

	println!("Cerrando programa");

	handle.join().unwrap();
}
