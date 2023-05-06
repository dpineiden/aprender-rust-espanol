use sqlx::postgres::PgPoolOptions;
use sqlx::Connection;
use sqlx::Postgres;
use sqlx::Pool;
use dotenv;
use std::env;
use test_db2::api::ciudad::{Ciudad,get_ciudades,get_city};
use test_db2::api::temperatura::{Temperatura,get_temperaturas};


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	// dotenv::dotenv().ok();
	// let key = "DATABASE_URL";
	// let url_db = env::var(key).unwrap();

	// println!("Usando env {}",&url_db);

	// //let url_db = "postgres://rust:oxido@localhost/test_rust";
	
	// // pool de conexiones
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&url_db).await?;

	// // obtener las ciudades
	// // select * from ciudad;
	// let ciudades = get_ciudades(&pool).await;
	// let temperaturas = get_temperaturas(&pool).await;

	// for t in temperaturas.iter() {
	// 	println!("{:?} -> {:?}", 
	// 			 t, 
	// 			 get_city(&ciudades, t.ciudad_id).unwrap().nombre);
	// }

	Ok(())
}



