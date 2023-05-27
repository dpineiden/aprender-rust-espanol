use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.
#[derive(Debug)]
struct Ciudad {
	pub id: i64,
	pub nombre: String,
	pub pais: String
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a connection pool
    //  for MySQL, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://rust:oxido@localhost/test_rust").await?;

    // // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let stream = sqlx::query_as::<_,Ciudad>("SELECT id, pais, nombre from ciudad")
        .fetch_one(&pool).await?;

	println!("{:?}", stream);

    Ok(())
}
