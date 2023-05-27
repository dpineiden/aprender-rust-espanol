
use sqlx::Postgres;
use sqlx::Pool;
use ejemplo_db2::api::ciudad::{Ciudad,
							insert_city, CiudadRow, 
							   delete_city_by_name, 
							   delete_city_by_id,
							   delete_cities_by_country};
use tracing_test::traced_test;

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


#[sqlx::test]
async fn update_ciudad(pool: Pool<Postgres>) -> sqlx::Result<()> {
    //let mut conn = pool.acquire().await?;
 	let ciudad = CiudadRow  {nombre:"Stgo".into(), pais:"Chile".into()};
	insert_city(&pool, &ciudad).await;
	let ciudad = CiudadRow  {nombre:"Valpo".into(), pais:"Argentina".into()};
	insert_city(&pool, &ciudad).await;
	let ciudad = CiudadRow  {nombre:"Conce".into(), pais:"Chile".into()};
	insert_city(&pool, &ciudad).await;
	let query = format!("select * from ciudad;");
	let ciudades = sqlx::query_as::<_,Ciudad>(&query).fetch_all(&pool)
				.await.unwrap();



	
    Ok(())
}


#[traced_test]
#[sqlx::test]
async fn delete_ciudad(pool: Pool<Postgres>) -> sqlx::Result<()> {
    //let mut conn = pool.acquire().await?;
 	let ciudad = CiudadRow  {nombre:"Santiago".into(), pais:"Chile".into()};
	insert_city(&pool, &ciudad).await;
	let ciudad = CiudadRow  {nombre:"Valparaíso".into(), pais:"Chile".into()};
	insert_city(&pool, &ciudad).await;
	let ciudad = CiudadRow  {nombre:"Concepción".into(), pais:"Chile".into()};
	insert_city(&pool, &ciudad).await;

	/*
	delete por Nombre de ciudad
	 */
	let nombre = "Santiago".to_string();
	delete_city_by_name(&pool, nombre).await;

	let query = format!("select * from ciudad;");
	let ciudades = sqlx::query_as::<_,Ciudad>(&query).fetch_all(&pool)
				.await.unwrap();

    assert_eq!(ciudades.len(), 2);    

	/*
	delete por id de ciudad
	 */
	delete_city_by_id(&pool, 2).await;

	let query = format!("select * from ciudad;");
	let ciudades = sqlx::query_as::<_,Ciudad>(&query).fetch_all(&pool)
				.await.unwrap();

    assert_eq!(ciudades.len(), 1);    
	/*
	Borrar por pais
	 */
	let ciudad = CiudadRow  {nombre:"Buenos Aires".into(), pais:"Argentina".into()};
	insert_city(&pool, &ciudad).await;
	let ciudad = CiudadRow  {nombre:"Mendoza".into(), pais:"Argentina".into()};
	insert_city(&pool, &ciudad).await;

	
	delete_cities_by_country(&pool, "Chile".into()).await;

	let query = format!("select * from ciudad;");
	let ciudades = sqlx::query_as::<_,Ciudad>(&query).fetch_all(&pool)
				.await.unwrap();

    assert_eq!(ciudades.len(), 2);    

	
    assert_eq!(1, 1);    
    Ok(())
}

/*
Al intentar borrar ciudad que tenga registro de temp en
temperaturas
tire un error

--> otro test

Otros propuestas:

- Hacer las update o modificacion de datos para temperatura y ciudad
- Formato -> fechas -> controlar formato
- Borrar ciudad -> borrar temperaturas primero

Otros propuestas:

- crear los comandos para update y delete de ciudad y temperatura

 */


