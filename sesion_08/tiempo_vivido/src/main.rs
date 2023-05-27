
/*
let naivedatetime_utc = NaiveDate::from_ymd_opt(2000, 1, 12).unwrap().and_hms_opt(2, 0, 0).unwrap();
let datetime_utc = DateTime::<Utc>::from_utc(naivedatetime_utc, Utc);
*/


mod persona {

	//TimeZone trait activa Utc.datetome_from_str
	use chrono::{DateTime, Utc, TimeZone, Duration};
	use serde::{Serialize, Deserialize};
	use chrono::format::ParseError;

	#[derive(Debug, Serialize, Deserialize)]
	pub struct Persona {
		nombre: String,
		fecha_nacimiento: DateTime<Utc>,		
	}

	impl Persona {
		const DATE_FORMAT:&str = "%Y/%m/%d %H:%M:%S";

		pub fn build(
			nombre:&str, 
			fecha_nacimiento: &str) -> Result<Self, ParseError> {
			let dt = format!("{fecha_nacimiento} 00:00:00");

			match Utc.datetime_from_str(&dt, Self::DATE_FORMAT){
				Ok(dt) => {
					let person = Self {
						nombre:nombre.into(), 
						fecha_nacimiento: dt.into()};
					Ok(person)
				},
				Err(e)=>{
					eprintln!("{}",e.to_string());
					Err(e)}
			}
		}

		pub fn get_fecha_nacimiento(&self) -> &DateTime<Utc> {
			&self.fecha_nacimiento
		}

		pub fn tiempo_vivido(&self, dt: &DateTime<Utc>) -> Duration{
			let duration = dt.signed_duration_since(self.fecha_nacimiento);
			Duration::days(duration.num_days())			
		}
	}

}
/*
Ejercicio: 

- Registrar nombre y fecha de nacimiento
- Calcular el tiempo vivido en segundos hasta el punto en que se
- ejecuta el programa
- guardar en csv los datos 

Considera crates:
- io
- fs
- chrono
- csv
*/
use std::io;
use persona::Persona;
use chrono::Utc;
use csv::Writer;
use serde::ser::StdError;

fn main()-> Result<(),Box<dyn StdError+'static>> {
    
	let mut wtr = Writer::from_path("personas.csv")?;
	let show_date_format = "año/mes/dia";
	let stdin = io::stdin();
	let now = Utc::now();
	let mut personas: Vec<Persona> = vec![];
	loop {
		let mut name = String::new();
		println!("Dame tu nombre:");
		io::stdin().read_line(&mut name)?;
		let name = name.trim();
		if name=="END" {break}
		let mut date = String::new();
		println!("Entregame tu fecha de nacimiento en formato {}", show_date_format);
		io::stdin().read_line(&mut date)?;
		let date = date.trim();
		if date=="END" {break}
		let persona = Persona::build(&name,&date)?;
		wtr.serialize(&persona)?;
		let tiempo_vivido = persona.tiempo_vivido(&now);
		println!("Persona {:?}", persona);
		println!("Días vividos {}", tiempo_vivido.num_days());
		personas.push(persona);
	}
	//  cerrar el writer csv
	wtr.flush()?;
	return Ok(());	
}
