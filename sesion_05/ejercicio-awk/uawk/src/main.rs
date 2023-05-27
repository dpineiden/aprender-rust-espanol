use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
// crate externo
use derivative::Derivative;

mod help;

/*
uso derivative: https://mcarton.github.io/rust-derivative/latest/index.html
*/

/*
Se prepara una entrada por entrada estandar (que permite encadenar
acciones)
y también el ingreso de paths
*/

#[derive(Debug)]
struct AWKOpts {
	separator: String,
	fields: Vec<u16>,
	show_row: bool,
	show_file: bool,
	show_nf: bool
}

// implementar new

impl AWKOpts {
	fn new(
		separator:String, 
		fields:Vec<u16>, 
		show_row:bool,
		show_file:bool,
		show_nf:bool)-> Self {

		AWKOpts {separator, fields, show_row, show_file, show_nf}
	}

	pub fn build(opts:HashMap<String,String>) -> Self {
		let separator = match opts.get("-F") {
			Some(sep)=> sep.to_string(),
			None=>" ".to_string()
		};
		//
		let fields = match opts.get("-C") {
			Some(selection)=> {
				let vector = selection
					.split(',')
					.map(|val|{
						// considers expressions like: n..m, n..NF
						val.to_string().trim().parse::<u16>().unwrap()
					}).collect();
				vector
			},
			None => vec![0]
		};
		
		/*
		let show_row = match opts.get("-R"){ 
			Some(_)=>true,
			None=>false
		};*/

		let show_row = opts.get("-R").is_some();

		//
		let show_file = opts.get("-I").is_some();

		let show_nf = opts.get("-T").is_some();

		Self::new(separator, fields, show_row, show_file, show_nf)
	}

	pub fn get_separator(&self)-> String {
		self.separator.clone()
	}

	pub fn get_fields(&self)-> Vec<u16> {
		self.fields.clone()
	}

	pub fn get_show_row(&self)-> bool {
		self.show_row
	}

	pub fn get_show_file(&self)-> bool {
		self.show_file
	}
	pub fn get_show_nf(&self)-> bool {
		self.show_nf
	}

}

// crear enum input : stdin/pathbuf
#[derive(Debug, Derivative, Eq, PartialEq)]
#[derivative(Hash)]
enum TextInput {
	FilePath(PathBuf),
	StdIn
}


/*
Leer de manera ordanada las opts y el Pathbuf asociado.
Si no tiene pathbuf será stdin
Para eso crear enum que considere ambos

Se prepara para cada Path una serie de args

Las opciones son:
- S :: define una entrada estandar
- F :: define separador
- C :: define los campos a mostrar
- R :: mostrar numero de linea
- T :: mostrar total de campos 
- O :: salida a archivo o stdout
- I :: mostrar info del archivo

Orden:
- S :: por defecto espacio ' '
- F :: por defecto toda la linea
- R :: por defecto no
- T :: no mostrar total campos
- O :: con encabezado meta

Cada opción entrega los valores con -[OP]="val"
*/
// portero :: gatekeeper
fn get_args(args:&[String]) -> Result<HashMap<TextInput, AWKOpts>, String> {
	let valid_options = vec!["-F","-C","-R","-T", "-O", "-S", "-I"];

	let mut opts_args:HashMap<String, String> = HashMap::new();//->
	//build-> AWKOpts
	// enum TextInput
	let mut paths_args:HashMap<TextInput, AWKOpts> =
		HashMap::new(); // controla lo que se muestra

	let mut new_pattern: bool = true;
	let mut stream = false;
	let help = "--help".to_string();

	for (i, arg) in args.iter().enumerate() {
		if arg == &help {
			help::read_help();
		}

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
				let awk_opts =  AWKOpts::build(opts_args);
                // recoger el path, si existe se incluye
				let mut pathbuf= PathBuf::new();
				let path = Path::new(&arg);
				// declaro que existira nuevo patron
			    new_pattern = true;
			    opts_args = HashMap::new();
				if path.exists() {
					pathbuf.push(arg);
					let key = TextInput::FilePath(pathbuf);
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
		let awk_opts =  AWKOpts::build(opts_args);
		let key = TextInput::StdIn;
		// hashmap que va a retornar
		paths_args.insert(key, awk_opts);
	};

	Ok(paths_args)
}


// exposición
fn show_line(nr:&usize, line:&String, opts: &AWKOpts) -> Result<(), String>
{
	// 	help::read_help();


	let str_fields =
		line.split(&opts.get_separator()).map(str::to_string).collect::<Vec<String>>();
	// field selection
	// la fila que se va a mostrar
	let mut new_line:Vec<String> = vec![];
	
	if opts.show_row {		
		new_line.push(nr.to_string());
	}
	
	// 1,3 -> 3,1 -> 2,0
	// 1,0,3 -> primer campo, linea completa, tercer campo
	// acumular segun el orden pedido
	for index in opts.fields.iter() {		
		if index!=&0  && index  <=  &(str_fields.len() as u16) {
			let word = &str_fields[*index as usize - 1];
			new_line.push(word.to_string());
		} else if index==&0 {
			new_line.push(line.into());
		}
	}

	if opts.show_nf {
		let nf = str_fields.len();
		let snf = format!("NF: {nf}");
		new_line.push(snf);
	}

	let text = new_line.join("|");
	println!("{}", text);
	Ok(())
}



fn main() {
	// estandar:: obtener lista de args como String
	let args:Vec<String> = env::args().collect();
	// ordenar, limpiar y asociar datos
	let opts = get_args(&args).unwrap();
	// shop opts
	for (key, opts) in opts.iter() {
		println!("key {:?} opts {:?}", key, opts);
		// por cada pathbuf, leer fuente por lineas

		match key {
			TextInput::StdIn => {
				let mut buffer = String::new();
				let stdin = io::stdin();
				let mut handle = stdin.lock();
				for (i, result_line) in handle.lines().enumerate() {
					match result_line {
						Ok(line)=>{
							show_line(&i, &line, opts).unwrap();
						},
						Err(err)=>eprintln!("Error en linea {}", i)
						}
				}

			},
			TextInput::FilePath(x) => {
				let mut file = fs::File::open(x).unwrap();
				let buf_reader = BufReader::new(file);

				for (i, result_line) in buf_reader.lines().enumerate() {
					match result_line {
						Ok(line)=>{
							if opts.get_show_file() {
								println!("Archivo a leer {:?}", &x)
							}
							show_line(&i, &line, opts).unwrap();
						},
						Err(err)=>println!("Error en linea")
						}
				}
				
			}
		};
	}
}
