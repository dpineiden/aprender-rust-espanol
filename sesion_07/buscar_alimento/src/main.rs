mod base;//  mod nombre -> archivo.rs
mod animales; // mod nombre -> directorio
mod bioma;
use crate::bioma::{Bioma, TipoBioma};
use crate::animales::*;
use crate::base::Accion;
use std::collections::HashMap;


type Ruta = Vec<Bioma>;


fn rutas_posibles(
	animales:&[Box<dyn Accion>], 
	rutas: &[Ruta])-> HashMap<String, Vec<usize>> {

	let mut result:HashMap<String, Vec<usize>> = HashMap::new();
	// iterar sobre animales
	// agregar a un vector todas aquellas rutas con todos los check en
	// true
	for box_animal in animales.iter() {
		// extrae el objeto de la 'caja' y entrega una referencia
		let animal = &*(*box_animal);
		let seleccion_biomas = rutas
			.iter()
			.enumerate()
			.filter_map(|(i,ruta)| {
		if ruta.iter().all(|bioma|  {bioma.check(animal)}){
			Some(i)
		} else {None}
		}).collect();
		result.insert(animal.get_nombre().to_string(), seleccion_biomas);
	}
	result
}

fn main() {
	// vectores de cantidad de biomas variables
	let ruta_1 = vec![
		Bioma {tipo:TipoBioma::Tierra, distancia:12.0},
		Bioma {tipo:TipoBioma::Agua, distancia:0.5},
		Bioma {tipo:TipoBioma::Tierra, distancia:2.0},
	];

	let ruta_2 = vec![
		Bioma {tipo:TipoBioma::Tierra, distancia:12.0},
		Bioma {tipo:TipoBioma::Agua, distancia:0.5},
		Bioma {tipo:TipoBioma::Aire, distancia:2.0},
		Bioma {tipo:TipoBioma::Agua, distancia:2.5},
	];

	let ruta_3 = vec![
		Bioma {tipo:TipoBioma::Tierra, distancia:12.0},
		Bioma {tipo:TipoBioma::Tierra, distancia:4.0},
		Bioma {tipo:TipoBioma::Tierra, distancia:1.0},
		Bioma {tipo:TipoBioma::Tierra, distancia:3.0},
		Bioma {tipo:TipoBioma::Aire, distancia:2.0},
	];

	// array de vectores
	let biomas = [ruta_1, ruta_2, ruta_3];


	let perro = Perro::default();
	let murcielago = Murcielago::default();
	let humano = Humano::default();
	let cuervo = Cuervo::default();
	let gaviota = Gaviota::default();
	let delfin = Delfin::default();
	let albacora = Albacora::default();
	let pato = Pato::default();
	let gallina = Gallina::default();


	let animales:Vec<Box<dyn Accion>> = vec![
		Box::new(perro),
		Box::new(murcielago),
		Box::new(humano), 
		Box::new(cuervo), 
		Box::new(gaviota), 
		Box::new(delfin),
		Box::new(albacora), 
		Box::new(pato), 
		Box::new(gallina)
	];


	let result = rutas_posibles(&animales, &biomas);
	println!("{:?}",result);

}
