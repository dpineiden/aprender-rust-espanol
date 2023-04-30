use sqlx::postgres::PgPoolOptions;
use sqlx::Connection;
use sqlx::Postgres;
use sqlx::Pool;
use dotenv;
use std::env;
use test_db2::api::temperatura::{Temperatura,get_temperaturas};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
	#[arg(short, long)]
	ciudad:Option<String>
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	let args = Options::parse();

	dotenv::dotenv().ok();
	let key = "DATABASE_URL";
	let url_db = env::var(key).unwrap();

	println!("Usando env {}",&url_db);

	//let url_db = "postgres://rust:oxido@localhost/test_rust";
	
	// pool de conexiones
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url_db).await?;

	// obtener las ciudades
	// select * from ciudad;
	let temperaturas = get_temperaturas(&pool, &args.ciudad).await;

	for t in temperaturas.iter() {
		println!("{:?}", t);
	}

	Ok(())
}



