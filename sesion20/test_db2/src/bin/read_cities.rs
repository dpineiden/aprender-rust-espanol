use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use sqlx::Pool;
use dotenv;
use std::env;
use test_db2::api::ciudad::{Ciudad,get_ciudades,get_city};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
	#[arg(short, long)]
	pais:Option<String>
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	let args = Options::parse();

	dotenv::dotenv().ok();
	let key = "DATABASE_URL";
	let url_db = env::var(key).unwrap();

	println!("Usando env {}",&url_db);
	println!("Pais buscado: {:?}", args.pais);
	//let url_db = "postgres://rust:oxido@localhost/test_rust";
	
	// pool de conexiones
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url_db).await?;

	// obtener las ciudades
	// select * from ciudad;

	let ciudades = get_ciudades(&pool, &args.pais).await;


	for (i,t) in ciudades.iter() {
		println!("{:?}",  t);
	}

	Ok(())
}



