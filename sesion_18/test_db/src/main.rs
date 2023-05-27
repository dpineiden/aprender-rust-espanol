use sqlx::postgres::PgPoolOptions;
use sqlx::Connection;
use futures::TryStreamExt;
use sqlx::Postgres;
use sqlx::Pool;
use sqlx::types::chrono::NaiveDate;

// etc.
#[derive(Debug,sqlx::FromRow,Clone)]
struct Ciudad {
	pub id: i32,
	pub nombre: String,
	pub pais: String
}

#[derive(Debug,sqlx::FromRow)]
struct Temperatura {
	pub id: i32,
	pub temperatura: f64,
	pub fecha: NaiveDate,
	pub ciudad_id: i32
}



#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
	let url_db = "postgres://rust:oxido@localhost/test_rust";

	// pool de conexiones
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url_db).await?;

	let cities = ciudades(&pool).await;

	let temps = temperaturas(&pool).await;

	for t in temps.iter() {
		println!("{:?} -> {:?}", t, get_city(&cities, t.ciudad_id).unwrap() )
	}

    Ok(())
}

fn get_city(cities:&Vec<Ciudad>, id:i32)->Option<Ciudad> {
	for ciudad in cities {
		if ciudad.id == id {
			return Some(ciudad.clone());
		}
	}
	None
}

async fn ciudades(pool:&Pool<Postgres>)->Vec<Ciudad>{
	let rows = sqlx::query_as::<_,Ciudad>("select * from ciudad;").fetch_all(pool).await.unwrap();

	for ciudad in rows.iter() {
		println!("{:?}", ciudad);
	}
	rows
}

async fn temperaturas(pool:&Pool<Postgres>) -> Vec<Temperatura>{
	let rows = sqlx::query_as::<_,Temperatura>("select * from temperatura;").fetch_all(pool).await.unwrap();

	for temp in rows.iter() {
		println!("{:?}", temp);
	}
	rows
}
