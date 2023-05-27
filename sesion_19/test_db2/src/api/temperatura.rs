use sqlx::types::chrono::NaiveDate;
use sqlx::Postgres;
use sqlx::Pool;


#[derive(Debug,Clone,sqlx::FromRow)]
pub struct Temperatura {
	pub id: i32,
	pub temperatura: f64,
	pub fecha: NaiveDate,
	pub ciudad_id: i32
}

pub async fn get_temperaturas(
	pool:&Pool<Postgres>, 
	ciudad:&Option<String>)->Vec<Temperatura> {

	match ciudad {
		Some(nombre)=>{
			let query = format!(
				"select * from temperatura join ciudad on
		ciudad.id=temperatura.ciudad_id where upper(ciudad.nombre)
		LIKE upper('{nombre}');");
			println!("Consultando con ciudad_ {}", &query);

			let temperaturas = sqlx::query_as::<_,Temperatura>(
				&query)
				.fetch_all(pool)
				.await.unwrap();

			temperaturas
		},
		None =>{
			let temperaturas = sqlx::query_as::<_,Temperatura>(
				"select * from temperatura")
				.fetch_all(pool)
				.await.unwrap();


			temperaturas
		}
	}
}

