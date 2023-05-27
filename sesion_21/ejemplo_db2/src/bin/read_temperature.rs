use sqlx::postgres::PgPoolOptions;
use sqlx::Connection;
use sqlx::Postgres;
use sqlx::Pool;
use dotenv;
use std::env;
use ejemplo_db2::api::temperatura::{Temperatura,get_temperaturas};
use clap::Parser;

use tracing::{info,warn,error, event, Level};
use tracing_subscriber;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
	#[arg(short, long)]
	ciudad:Option<String>
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	tracing_subscriber::fmt::init();
	//console_subscriber::init();
	let args = Options::parse();

	dotenv::dotenv().ok();
	let key = "DATABASE_URL";
	let url_db = env::var(key).unwrap();

	info!("Usando env {}",&url_db);

	//let url_db = "postgres://rust:oxido@localhost/test_rust";
	
	// pool de conexiones
	// pool de conexiones
    match PgPoolOptions::new()
        .max_connections(5)
        .connect(&url_db).await {
			Ok(pool)=>{

				 let temperaturas = get_temperaturas(&pool, &args.ciudad).await;

				 for t in temperaturas.iter() {
					 info!("{:?}", t);
				 }

			},
			Err(err)=> {event!(
				Level::ERROR,"No se pudo activar el pool de conexiones");}
		};


	Ok(())
}



