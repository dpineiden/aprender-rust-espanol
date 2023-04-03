use std::env;
use std::io::{self, BufRead, BufReader};
use std::fs;

mod base;
mod read_args;
mod match_lines;

fn main() {
	// leer la entrada como parámetros
	let args:Vec<String> = env::args().collect();
	// ordenar, limpiar y asociar datos
	let opts = read_args::get_args(&args).unwrap();
	for (key, opts) in opts.iter() {
		match key {
			base::TextInput::StdIn => {
				let stdin = io::stdin();
				let mut handle = stdin.lock();
				for (i, result_line) in handle.lines().enumerate() {
					match result_line {
						Ok(line)=>{
							match_lines::show_line(&i, &line, opts).unwrap();
						},
						Err(err)=>eprintln!("Error en linea {}", i)
						}
				}

			},
			base::TextInput::FilePath(x) => {
				let mut file = fs::File::open(x).unwrap();
				let buf_reader = BufReader::new(file);

				for (i, result_line) in buf_reader.lines().enumerate() {
					match result_line {
						Ok(line)=>{
							match_lines::show_line(&i, &line, opts).unwrap();
						},
						Err(err)=>println!("Error en linea")
						}
				}
				
			}
		};
	}

}
