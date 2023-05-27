use sqlx::postgres::PgPoolOptions;
use sqlx::Connection;
use sqlx::Postgres;
use sqlx::Pool;
use sqlx::types::chrono::NaiveDate;

#[derive(Debug,Clone,sqlx::FromRow)]
struct Ciudad {
	pub id: i32,
	pub nombre: String,
	pub pais: String
}


#[derive(Debug,Clone,sqlx::FromRow)]
struct Temperatura {
	pub id: i32,
	pub temperatura: f64,
	pub fecha: NaiveDate,
	pub ciudad_id: i32
}



#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	let url_db = "postgres://rust:oxido@localhost/test_rust";
	
	// pool de conexiones
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url_db).await?;

	// obtener las ciudades
	// select * from ciudad;
	let ciudades = get_ciudades(&pool).await;
	let temperaturas = get_temperaturas(&pool).await;

	for t in temperaturas.iter() {
		println!("{:?} -> {:?}", 
				 t, 
				 get_city(&ciudades, t.ciudad_id).unwrap().nombre);
	}

	Ok(())
}


async fn get_ciudades(pool:&Pool<Postgres>)->Vec<Ciudad>{
	let ciudades = sqlx::query_as::<_,Ciudad>(
		"select * from ciudad;").fetch_all(pool)
		.await.unwrap();
	
	for ciudad in ciudades.iter() {
		println!("{:?}", ciudad);
	}

	ciudades
}

async fn get_temperaturas(pool:&Pool<Postgres>)->Vec<Temperatura> {
	let temperaturas = sqlx::query_as::<_,Temperatura>(
		"select * from temperatura")
		.fetch_all(pool)
		.await.unwrap();
	
	for temp in temperaturas.iter() {
		println!("{:?}", temp);
	}

	temperaturas
}

fn get_city(cities:&Vec<Ciudad>, id:i32) -> Option<Ciudad> {
	for ciudad in cities {
		if ciudad.id == id {
			return Some(ciudad.clone());
		}
	}
	None
}
