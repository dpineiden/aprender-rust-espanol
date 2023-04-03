use std::time::{Duration,Instant};
use std::thread::sleep;
use std::io;
use std::error::Error;


mod persona {
	// chrono
	use chrono::{DateTime,Utc,TimeZone,Duration};
	use chrono::format::ParseError;
	use serde::{Serialize,Deserialize};

	#[derive(Debug,Serialize,Deserialize)]
	pub struct Persona {
		nombre:String,
		fecha_nacimiento: DateTime<Utc>
	}

	impl Persona {
		const DATE_FORMAT:&str = "%Y/%m/%d %H:%M:%S";

		pub fn build(
			nombre:&str, 
			fecha_nacimiento:&str) -> Result<Self, ParseError> {
			let dt = format!("{fecha_nacimiento} 00:00:00");

			match Utc.datetime_from_str(&dt, Self::DATE_FORMAT) {
				Ok(dtn)=>{
					let persona = Self {
						nombre:nombre.into(),
						fecha_nacimiento: dtn
					};
					Ok(persona)
				},
				Err(e)=>{
					eprintln!("{}", e.to_string());
					Err(e)
				}
			}

		}

	   pub fn tiempo_vivido(&self, dt:&DateTime<Utc>) -> Duration {
		   let duration = dt.signed_duration_since(self.fecha_nacimiento);
		   Duration::days(duration.num_days())
	   }


	}

}

/*
Ejercicio: 

- Registrar nombre y fecha de nacimiento
- Calcular el tiempo vivido en -> dias
- ejecuta el programa
- guarda csv

Crates:

- io
- fs
- chrono
- serde
- csv

*/

use persona::Persona;
use chrono::Utc;
use csv::Writer;

fn main() -> Result<(), Box<dyn Error+'static>>{

	let mut wtr = Writer::from_path("personas.csv")?;
	let stdin = io::stdin();
	//let mut personas:Vec<String> => vec![];
	let today  = Utc::now();
	let show_date_format = "año/mes/día";
	loop {
		let mut name = String::new();
		println!("Dame tu nombre:");
		stdin.read_line(&mut name)?;
		let name = name.trim();
		println!("{}",name);
		if name=="END"{break}
		println!(
			"Dame la fecha de nacimiento en formato {}",
	show_date_format);
		let mut date = String::new();
		stdin.read_line(&mut date)?;
		let date = date.trim();
		if date=="END"{break}
		// crear persona
		let persona = Persona::build(&name, &date)?;
		println!("{:?}", &persona);
		let dias_vividos = persona.tiempo_vivido(&today);
		println!("Dias vividos {}", dias_vividos.num_days());
		wtr.serialize(&persona)?;
		wtr.flush()?;
	}
	return Ok(());
}
