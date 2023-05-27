use sqlx::Postgres;
use sqlx::Pool;
use tracing::{info, error, debug};

use std::{
	collections::HashMap,
	path::{PathBuf},
	fs::File,
	error::Error
};

#[derive(Debug,Clone,sqlx::FromRow)]
pub struct Ciudad {
	pub id: i32,
	pub nombre: String,
	pub pais: String
}

#[derive(Debug,Clone,serde::Deserialize)]
pub struct CiudadRow {
	pub nombre: String,
	pub pais: String
}


#[tracing::instrument]
pub async fn read_csv_cities(
	pool: &Pool<Postgres>,
	path:&PathBuf) ->Result<(),String>{
	if path.exists(){
		let file = File::open(path).unwrap();
		let mut rdr = csv::Reader::from_reader(file);
		for result in rdr.deserialize::<CiudadRow>() {
			match result {
				Ok(record)=>{
					insert_city(pool, &record).await;
				},
				Err(err) =>{
					error!("{}",err.to_string());
				}
			}
			//let record:CiudadRow = result?;
		}
	} else {
		error!("Path cannot be found {:?}",path);
		return Err("Path cannot be found".into());
	}
	Ok(())
}


#[tracing::instrument]
pub async fn insert_city(
	pool: &Pool<Postgres>,
	ciudad: &CiudadRow
){
	let query = format!("insert into ciudad (pais, nombre) values ('{0}','{1}');", ciudad.pais, ciudad.nombre);
	match sqlx::query(&query).execute(pool).await {
		Ok(result)=>{
			info!("Resultado de insert {:?}", &result);
		},
		Err(err)=>{
			error!("Error al cargar el dato {:?}", err);
		}
	};
}

#[tracing::instrument]
pub async fn get_ciudades(
	pool:&Pool<Postgres>,
	pais:&Option<String>)->HashMap<i32, Ciudad>{
	let ciudades = match pais {
		Some(nombre) => {
			let query = format!("select * from ciudad where upper(pais) LIKE upper('{nombre}');");
			info!("Consultando con pais_ {}", &query);
			let ciudades = sqlx::query_as::<_,Ciudad>(&query).fetch_all(pool)
				.await.unwrap();
			ciudades
		},
		None => {
			info!("Consultando todos los países");
			let ciudades = sqlx::query_as::<_,Ciudad>(
				"select * from ciudad;").fetch_all(pool)
				.await.unwrap();
			ciudades
		}

	};

	let mut result = HashMap::new();
	for ciudad in ciudades.into_iter(){
		result.insert(ciudad.id, ciudad);
	}
	info!("Mapa de id:ciudad cargado correctamente");
	result
}


#[tracing::instrument]
pub fn get_city(cities:&Vec<Ciudad>, id:i32) -> Option<Ciudad> {
	for ciudad in cities {
		if ciudad.id == id {
			return Some(ciudad.clone());
		}
	}
	None
}




