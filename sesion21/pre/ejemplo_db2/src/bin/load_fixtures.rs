use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use sqlx::Pool;
use dotenv;
use std::env;
use tracing::{error, info};


use clap::Parser;
use std::{
	path::PathBuf,
};
use ejemplo_db2::api::ciudad::{read_csv_cities, get_ciudades};
use ejemplo_db2::api::temperatura::{read_csv_temperatura};



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
	#[arg(short, long)]
	directory:Option<String>
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	tracing_subscriber::fmt::init();

	let args = Options::parse();

	dotenv::dotenv().ok();
	let key = "DATABASE_URL";
	let url_db = env::var(key).unwrap();

	println!("Usando env {}",&url_db);
	println!("Directorio: {:?}", args.directory);
	//let url_db = "postgres://rust:oxido@localhost/test_rust";
	
	// pool de conexiones
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url_db).await?;

	// obtener las ciudades
	// vamos a leer el csv de ciudad
	match args.directory {
		Some(path)=>{
			// path: "./fixtures"
			let ciudades_path:PathBuf = [path.clone(), "ciudades.csv".into()].iter().collect();
			read_csv_cities(&pool,&ciudades_path).await.unwrap();
			let ciudades = get_ciudades(&pool, &None).await;
			let temp_path:PathBuf = [path, "temperaturas.csv".into()].iter().collect();
			read_csv_temperatura(&pool, &ciudades, &temp_path).await.unwrap();
		},
		None=>{
			let ciudades_path:PathBuf = [
				"./fixtures", "ciudades.csv".into()].iter().collect();
			match read_csv_cities(&pool,&ciudades_path).await {
			Ok(_)=>{
				let ciudades = get_ciudades(&pool, &None).await;
				let temp_path:PathBuf = [
					"./fixtures", "temperaturas.csv".into()].iter().collect();
				read_csv_temperatura(&pool, &ciudades,
									 &temp_path).await.unwrap();
			},
				Err(err) => {
					error!("Fixtures Cannot be loaded, {}", err.to_string());
				}
				}

		}
	}


	Ok(())
}



