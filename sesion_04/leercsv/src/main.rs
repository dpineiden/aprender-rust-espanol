use std::env;
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Temp {
	dia: String,
	fahrenheit: f32,
}

fn main() {
    println!("Hello, world!");
	let args = env::args().collect::<Vec<String>>();
	let csv = Path::new(&args[1]);
	println!("Ruta csv {:?}, existe? {}", csv, csv.exists());
	if csv.exists() {
		let file = fs::File::open(csv).unwrap();
		let mut reader = csv::Reader::from_reader(file);
		for record in reader.deserialize() {
			let linea:Temp = record.unwrap();
			println!("Valor: {:?}", &linea);
		}
		
	} 
	else {
		eprintln!("El archivo {:?} no existe", &csv);}
}
