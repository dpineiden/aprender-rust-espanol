use creando_macros::add;
use creando_macros::vectorcito;
use creando_macros::crear_ast;
use std::path::Path;
mod ejemplo;
use ejemplo::crear_persona;

fn main() {
    println!("Hello, world! {}", add(1,2));
	let numeros = vectorcito![1,2,3,5,6,7];
	println!("{:?}", numeros);
	let path = Path::new("./src/ejemplo.rs");
	crear_ast(path);

	let persona = crear_persona(
		"David".into(), 
		"Pineda".into(), 
		39);

	if persona.esta_vivo(){
		println!("{:?} está vivo", persona);
	}
}
