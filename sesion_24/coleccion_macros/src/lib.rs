#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::collections::HashMap;
use std::path::Path;


// expone en publico la macro
#[macro_export]
macro_rules! mostrar_metadata {
	($struct:ident)=>{
		println!("{}", stringify!($struct));
		let p = $struct::default();
		println!("Default {:?}", p);	
		println!("Clon {:?}", p.clone());
	}
}


#[macro_export]
macro_rules! medir_tiempo {
	($name:expr, $block:block) =>{
		{
		let start = std::time::Instant::now();
		let result = {$block};
		let duration = start.elapsed();
		println!("Tiempo de ejecución {} -> {:?}", $name, duration);
		result
		}
	}

}


// T asociado con trait: Deserialize
pub fn leer_csv<T: for<'de> serde::Deserialize<'de>>(path:&Path)-> Result<HashMap<String,T>, String>{
	let mut hashmap: HashMap<String, T> = HashMap::new();
	let file = File::open(path).unwrap();
	let mut rdr = csv::Reader::from_reader(file);
	let mut counter = 0;
	for result in rdr.deserialize::<T>() {
		match result  {
			Ok(record)=>{
				hashmap.insert(format!("{counter}"),record);
				/* en csv grandes */
				// hacer una operacion sobre el dato
				// usar generador -> yield
				// channels -> while/loop
				// envie el dato y el loop lo opere
			},
			Err(err)=>{
				eprintln!("{}", err.to_string());
				return Err("Some error".into())
			}
		};
		counter+=1;
	}
	Ok(hashmap)
}

#[macro_export]
macro_rules! zip_to_hashmap {
	($keys:expr, $values:expr) => {{
		let mut hashmap = HashMap::new();
		let mut iter_values = $values.into_iter();
		for key in $keys {
			if let Some(value)= iter_values.next() {
				hashmap.insert(key,value);
			} else {
				break;
			}
		}
		hashmap
	}}
}



