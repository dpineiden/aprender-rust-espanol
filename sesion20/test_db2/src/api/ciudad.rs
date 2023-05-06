use sqlx::Postgres;
use sqlx::Pool;
use std::collections::HashMap;

#[derive(Debug,Clone,sqlx::FromRow)]
pub struct Ciudad {
	pub id: i32,
	pub nombre: String,
	pub pais: String
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
