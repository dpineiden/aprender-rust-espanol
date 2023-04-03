use std::path::{Path};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;


fn main() {
	let archivos = Path::new("./../archivos");
    println!("Existe archivos {}", archivos.try_exists().unwrap());
	println!("Es un directorio {}", archivos.is_dir());

	let mut archivos_org:Vec<String> = Vec::new();

	for entry in archivos.read_dir()
		.expect("No ha sido posible leer directorio") {
			if let Ok(entry) = entry {
				let file = entry.path();
				match file.extension(){
					Some(ext) => {
						if ext == "org" {
							let archivo = file.to_str().unwrap().to_string();
							println!("Es org {}",	archivo);
							archivos_org.push(archivo.clone());	
						}
					},
					None => {}
				};	

			}

		};

	println!("===========");
	/* uso de args simple */
	for (i, argument) in env::args().enumerate() {
		println!("Entrada:: {} {}", i, argument);
	}

	let args:Vec<String> = env::args().collect();
	let nombre_archivo = &args[1];

	println!("Nombre del archivo {}", nombre_archivo);
	
	for archivo in archivos_org.iter() {
		println!("Archivo org:: {}", archivo);
		let path = Path::new(archivo);
		println!("Archivo existe {}", path.try_exists().unwrap());
		println!("Filename {}", path.file_name().unwrap().to_str().unwrap());
		let filename = path.file_stem().unwrap().to_str().unwrap();
		if nombre_archivo == filename {
			println!("Leyendo archivo {}", filename);
			// path
			//let contents = fs::read_to_string(&path).unwrap();
			let file = fs::File::open(&path).unwrap();
			let mut buf_reader = BufReader::new(file);
			let mut contents = String::new();
			buf_reader.read_to_string(&mut contents).unwrap();
			println!("{}", contents);
		}
	}
}
