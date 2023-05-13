
use sqlx::Postgres;
use sqlx::Pool;
use ejemplo_db2::api::ciudad::{Ciudad,
							insert_city, CiudadRow};

//type PgPool = Pool<Postgres>;

#[sqlx::test]
async fn basic_test(_pool: Pool<Postgres>) -> sqlx::Result<()> {
    //let mut conn = pool.acquire().await?;
    assert_eq!(1, 1);    
    Ok(())
}


#[sqlx::test]
async fn insert_one_test(pool: Pool<Postgres>) -> sqlx::Result<()> {
	let ciudad = CiudadRow  {nombre:"Santiago".into(), pais:"Chile".into()};
	insert_city(&pool, &ciudad).await;
	let query = format!("select * from ciudad;");
	let ciudades = sqlx::query_as::<_,Ciudad>(&query).fetch_all(&pool)
				.await.unwrap();
    assert_eq!(ciudades.len(), 1);    
    Ok(())
}
