use std::path::{Path, PathBuf};
use std::collections::HashMap;
use super::base;

/*
Opciones:
- E :: estricto
- R :: mostrar número de línea
- S :: activar lectura de stream
*/
pub fn get_args(args:&[String]) -> Result<HashMap<base::TextInput, base::FileOpts>, String> {
	let valid_options = vec!["-S", "-R", "-E"];

	let mut opts_args:HashMap<String, String> = HashMap::new();//->
	//build-> AWKOpts
	// enum TextInput
	let mut paths_args:HashMap<base::TextInput, base::FileOpts> = HashMap::new(); 
	// controla lo que se muestra

	let mut new_pattern: bool = true;
	let mut stream = false;
	for (i, arg) in args.iter().enumerate() {

		if i > 0 {
			if arg.starts_with('-') {
				new_pattern = false;
				let head = &arg[0..2];
				let tail = &arg[2..];
				if valid_options.contains(&head) {
					if head == "-S" {
						stream = true;
					}
					// hashmap -> llave con el valor
					// llenamos las opciones para cada patron
					opts_args.insert(head.into(), tail.into());
				} else {
					// si existe alguna opcion inválida se levanta error
				    let vals = valid_options.clone().join(",");
				    let msg = format!("Some option not valid, avaiable options [{vals}]");
				    return Err(msg);				}
			} else  {
				// build opts :: construimos las opciones
				let awk_opts =  base::FileOpts::build(opts_args);
                // recoger el path, si existe se incluye
				let mut pathbuf= PathBuf::new();
				let path = Path::new(&arg);
				// declaro que existira nuevo patron
			    new_pattern = true;
			    opts_args = HashMap::new();
				if path.exists() {
					pathbuf.push(arg);
					let key = base::TextInput::FilePath(pathbuf);
					// hashmap que va a retornar
					paths_args.insert(key, awk_opts);
				} else {
					// salida de error estandar
					eprintln!("{}", &format!("Path {:?} doesn't exists", path.to_str().unwrap()));
				}
			
			}

		}
	}
	/*
	Si new_pattern is true and opt_args not empty
	estas settings son para stdin
	*/
	if !new_pattern  && stream {
		let f_opts =  base::FileOpts::build(opts_args);
		let key = base::TextInput::StdIn;
		// hashmap que va a retornar
		paths_args.insert(key, f_opts);
	};

	Ok(paths_args)
}
