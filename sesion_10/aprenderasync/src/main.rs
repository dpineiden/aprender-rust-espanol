use futures::executor::block_on;

async fn aprender_cancion()->String {
	"La cucaracha".to_owned()
}

async fn cantar_cancion(cancion: &str){ 
	// busqueda a base de datos
	// let lirics = buscar_en_db(cancion).await.unwrap();
	let lirics = "La cucaracha, la cucaracha\nya no puede caminar";
	println!("Título: {}", cancion);
	println!("{}", lirics);
}


async fn danzar(){
	println!("{}","¡Estamos danzando!");
}

async fn aprender_y_cantar() {
	let cancion = aprender_cancion().await;
	cantar_cancion(&cancion).await;
}

// Future
async fn async_main() {
	let f1 = aprender_y_cantar();
	let f2 = danzar();
	futures::join!(f1,f2);
}

fn main() {
	block_on(async_main())
}
