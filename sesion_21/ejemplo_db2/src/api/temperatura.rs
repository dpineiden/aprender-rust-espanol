use sqlx::types::chrono::NaiveDate;
use sqlx::Postgres;
use sqlx::Pool;
use crate::api::ciudad::Ciudad;
use tracing::{info,error,debug};

use std::{
	collections::HashMap,
	path::{PathBuf},
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

/*
Ejercicio: hacer read_csv como un generic con T 
en
tool.rs
*/
#[tracing::instrument]
pub async fn read_csv_temperatura(
	pool: &Pool<Postgres>,
	ciudades:&HashMap<i32, Ciudad>,
	path:&PathBuf) ->Result<(),String>{

	if path.exists(){
		let file = File::open(path).unwrap();
		let mut rdr = csv::Reader::from_reader(file);
		for result in rdr.deserialize::<TemperaturaRow>() {
			match result {
				Ok(record)=>{
					insert_temperatura(pool, ciudades, &record).await;
				},
				Err(err) => {
					error!("{}", err.to_string());
				}
			};
		}
	} else {
		error!("Path cannot be found {:?}", path);
		return Err("Path cannot be found".into())
	}
	Ok(())
}


#[tracing::instrument]
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

#[tracing::instrument]
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
					info!("Resultado de insert {:?}", &result);
				},
				Err(err)=>{
					error!("Error al cargar el dato {:?}", err);
				}
			};
		}
		None=>{
			error!("Error, la ciudad para esta temperatura no existe");
		}
	}
}



#[tracing::instrument]
pub async fn get_temperaturas(
	pool:&Pool<Postgres>, 
	ciudad:&Option<String>)->Vec<Temperatura> {

	match ciudad {
		Some(nombre)=>{
			let query = format!(
				"select * from temperatura join ciudad on
		ciudad.id=temperatura.ciudad_id where upper(ciudad.nombre)
		LIKE upper('{nombre}');");
			info!("Consultando con ciudad_ {}", &query);

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

