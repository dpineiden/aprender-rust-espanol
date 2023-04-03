use std::env;
use std::path::{Path, PathBuf};
//use std::ffi::OsStr;

fn get_args(args:&Vec<String>) -> Result<(Vec<char>, Vec<PathBuf>), &'static str> {
	let valid_options = vec!['l','s','d','r'];

	let mut opts_args:Vec<char> = Vec::new();
	let mut paths_args:Vec<PathBuf> = Vec::new();

	for (i, arg) in args.iter().enumerate() {
		if i > 0 {
			if arg.starts_with('-') {
				let opts:Vec<char> = arg.replace('-', "").chars().collect();
				if opts.iter().all(|op| valid_options.clone().contains(op) ) {
					for op in opts.iter() {
						opts_args.push(*op)
					}
				} else {
					return Err("Some option not valid, avaiable options [l,s,d,r]");
				}
			} else  {
				let mut pathbuf= PathBuf::new();
				let path = Path::new(&arg);
				if path.exists() {
					pathbuf.push(arg);
					paths_args.push(pathbuf);
				} else {
					eprintln!("{}", &format!("Path {:?} doesn't exists", path.to_str().unwrap()))
				}
			
			}

		}
	}

	if paths_args.is_empty() {
		let mut pathbuf= PathBuf::new();
		pathbuf.push("./");
		paths_args.push(pathbuf);
	}

	Ok((opts_args, paths_args))
}

fn main() {
	// se definen las opciones disponibles
	let args:Vec<String> = env::args().collect();
	let (opts, paths) = get_args(&args).unwrap();
	// de args se toma desde la posición 1 en adelante
	// se recolectan los paths que no comienzan con '-'
	// todas las opciones con '-' que estén en options se aceptan
	// si no están en la lista, se rechaza operación
	// valdrá instaurar un orden? -opts path -opts2 path
	// veamos que saldría
	// chequear 

	for opt in opts.iter() {
		println!("Opcion {}", opt);
	}

	for pb in paths.iter() {
		println!("Path {}", pb.to_str().unwrap());
	}
}
