use sqlx::postgres::PgPoolOptions;
use dotenv;
use std::env;
use ejemplo_db2::api::ciudad::get_ciudades;
use clap::Parser;
use tracing::{info,warn,error, event, Level};
use tracing_subscriber;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
	#[arg(short, long)]
	pais:Option<String>
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
	tracing_subscriber::fmt::init();
	//console_subscriber::init();

	let args = Options::parse();
	info!("Se ha cargado args 'correctamente'");

	dotenv::dotenv().ok();
	let key = "DATABASE_URL";
	let url_db = env::var(key).unwrap();

	warn!("Usando env {}",&url_db);
	warn!("Pais buscado: {:?}", args.pais);
	//let url_db = "postgres://rust:oxido@localhost/test_rust";
	
	// pool de conexiones
    match PgPoolOptions::new()
        .max_connections(5)
        .connect(&url_db).await {
			Ok(pool)=>{
				 // obtener las ciudades
				 // select * from ciudad;

				 let ciudades = get_ciudades(&pool, &args.pais).await;


				 for (i,t) in ciudades.iter() {
					 info!("{:?}",  t);
				 }
			},
			Err(err)=> {event!(
				Level::ERROR,"No se pudo activar el pool de conexiones");}
		};


	Ok(())
}



