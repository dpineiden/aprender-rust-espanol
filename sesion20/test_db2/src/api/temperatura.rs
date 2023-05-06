use sqlx::types::chrono::NaiveDate;
use sqlx::Postgres;
use sqlx::Pool;
use crate::api::ciudad::Ciudad;

use std::{
	collections::HashMap,
	path::{Path,PathBuf},
	fs::File,
	error::Error
};


#[derive(Debug,Clone,sqlx::FromRow)]
pub struct Temperatura {
	pub id: i32,
	pub temperatura: f64,
	pub fecha: NaiveDate,
	pub ciudad_id: i32
}


#[derive(Debug,Clone,serde::Deserialize)]
pub struct TemperaturaRow {
	pub temperatura: f64,
	pub fecha: String,
	pub ciudad: String
}

pub async fn read_csv_temperatura(
	pool: &Pool<Postgres>,
	ciudades:&HashMap<i32, Ciudad>,
	path:&PathBuf) ->Result<(),Box<dyn Error>>{
	let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
	for result in rdr.deserialize() {
		let record:TemperaturaRow = result?;
		insert_temperatura(pool, ciudades, &record).await;
	}
	Ok(())
}


fn get_ciudad_id(
	ciudades:&HashMap<i32,	Ciudad>,
	nombre:&String)->Option<i32> {
	/*
	Propuesta: Usar un hasmap inverso: (ciuda,pais) -> id
	 */
	for (i, ciudad) in ciudades.iter() {
		if &ciudad.nombre == nombre {
			return Some(i.clone());
		}
	}
	None
}

pub async fn insert_temperatura(
	pool: &Pool<Postgres>,
	ciudades:&HashMap<i32, Ciudad>,
	temp: &TemperaturaRow
){
	let opt_ciudad_id = get_ciudad_id(ciudades, &temp.ciudad);
	
	match opt_ciudad_id {
		Some(ciudad_id)=>{
			let query = format!("insert into temperatura (temperatura, fecha,
			ciudad_id) values ('{0}',to_date('{1}', 'dd/mm/yyyy'), '{2}');", 
								temp.temperatura,
								temp.fecha, 
								ciudad_id);
			match sqlx::query(&query).execute(pool).await {
				Ok(result)=>{
					println!("Resultado de insert {:?}", &result);
				},
				Err(err)=>{
					eprintln!("Error al cargar el dato {:?}", err);
				}
			};
		}
		None=>{
			eprintln!("Error, la ciudad para esta temperatura no existe");
		}
	}
}



pub async fn get_temperaturas(
	pool:&Pool<Postgres>, 
	ciudad:&Option<String>)->Vec<Temperatura> {

	match ciudad {
		Some(nombre)=>{
			let query = format!(
				"select * from temperatura join ciudad on
		ciudad.id=temperatura.ciudad_id where upper(ciudad.nombre)
		LIKE upper('{nombre}');");
			println!("Consultando con ciudad_ {}", &query);

			let temperaturas = sqlx::query_as::<_,Temperatura>(
				&query)
				.fetch_all(pool)
				.await.unwrap();

			temperaturas
		},
		None =>{
			let temperaturas = sqlx::query_as::<_,Temperatura>(
				"select * from temperatura")
				.fetch_all(pool)
				.await.unwrap();


			temperaturas
		}
	}
}

