use sqlx::Postgres;
use sqlx::Pool;

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

pub async fn read_csv_cities(
	pool: &Pool<Postgres>,
	path:&PathBuf) ->Result<(),Box<dyn Error>>{
	let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
	for result in rdr.deserialize() {
		let record:CiudadRow = result?;
		insert_city(pool, &record).await;
	}
	Ok(())
}


pub async fn insert_city(
	pool: &Pool<Postgres>,
	ciudad: &CiudadRow
){
	let query = format!("insert into ciudad (pais, nombre) values ('{0}','{1}');", ciudad.pais, ciudad.nombre);
	match sqlx::query(&query).execute(pool).await {
		Ok(result)=>{
			println!("Resultado de insert {:?}", &result);
		},
		Err(err)=>{
			eprintln!("Error al cargar el dato {:?}", err);
		}
	};
}

pub async fn get_ciudades(
	pool:&Pool<Postgres>,
	pais:&Option<String>)->HashMap<i32, Ciudad>{
	let ciudades = match pais {
		Some(nombre) => {
			let query = format!("select * from ciudad where upper(pais) LIKE upper('{nombre}');");
			println!("Consultando con pais_ {}", &query);
			let ciudades = sqlx::query_as::<_,Ciudad>(&query).fetch_all(pool)
				.await.unwrap();
			ciudades
		},
		None => {
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
	result


}

pub fn get_city(cities:&Vec<Ciudad>, id:i32) -> Option<Ciudad> {
	for ciudad in cities {
		if ciudad.id == id {
			return Some(ciudad.clone());
		}
	}
	None
}




