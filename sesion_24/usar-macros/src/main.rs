use clap::Parser;
use std::path::Path;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

use coleccion_macros::{mostrar_metadata, leer_csv, medir_tiempo, zip_to_hashmap};

#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
	#[arg(short, long)] // -p --path
	path: Option<String>,
	#[arg(short, long)] 
	tipo: Option<u8>
}

#[derive(Debug,Clone, serde::Deserialize,Default)]
struct Ciudad {
	pub nombre: String,
	pub pais: String
}


#[derive(Debug,Clone, serde::Deserialize,Default)]
struct Temperatura {
	pub temperatura: f64,
	pub fecha: String,
	pub ciudad: String
}


fn main() {
	let tipos = ["ciudad", "temperatura"];
	for (i,tipo) in tipos.iter().enumerate() {
		println!("{} -> {}", i, tipo);
	}
	// toma los argumentos de entrada en un comnado
	// cli
	let args = Args::parse();
	println!("{:?}", args);
    println!("Hello, world!");

	match args.path {
		Some(spath) => {
		   let path = Path::new(&spath);

		   if path.exists() {
			   // leer el archivo y mostrar los datos
			   println!("Leyendo csv {:?}", path);
			   mostrar_metadata!(Temperatura);
			   match args.tipo {
				   Some(v) if v==0=>{
					   // tiempo que demora la acción:::
					  let ciudades = medir_tiempo!("csv ciudad",{
						  leer_csv::<Ciudad>(&path).unwrap()
					  }
					  );

					  for (i,v) in ciudades.into_iter() {
						  println!("{:?} -> {:?}", i,v);
					  }
				   },
				   Some(v) if v==1=>{
					  let mut temperaturas = medir_tiempo!("csv
						  temperaturas",{
							  leer_csv::<Temperatura>(&path).unwrap()
						  });

					  for (i,v) in temperaturas.into_iter() {
						  println!("{:?} -> {:?}", i,v);
					  }
				   },
				   Some(_) =>{ 
					   eprintln!("No existe tipo mayor a 1");
				   }
				   None => {
					   eprintln!("No hay tipo para leer csv");
				   }

			   }


		   }
		   else {
			   eprintln!("Path {:?} no existe", path);
		   }


		},
		None => {
			eprintln!("No entregaste valor a path");
		}

	}

    let keys = ["key1", "key2", "key3", "key4"];
    let values = [1, 2, 3];

    let hashmap = zip_to_hashmap!(keys, values);

    println!("{:?}", hashmap);
	

}
