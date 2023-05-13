use sqlx::postgres::PgPoolOptions;
use dotenv;
use std::env;
use ejemplo_db2::api::ciudad::get_ciudades;
use clap::Parser;
use tracing::{span, event, info, Level};
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

	//let span = span!(Level::TRACE, "load-fixtures-span");
	//let _enter = span.enter();

	let args = Options::parse();
	info!("Se ha cargado 'args' correctamente!");

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


	for (_i,t) in ciudades.iter() {
		println!("{:?}",  t);
	}

	Ok(())
}



