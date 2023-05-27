mod base;
mod animales;
mod bioma;
use crate::bioma::{Bioma, TipoBioma};
use crate::animales::perro::Perro;

fn main() {
	let ruta_1 = [
		Bioma {tipo:TipoBioma::Tierra, distancia:12.0},
		Bioma {tipo:TipoBioma::Agua, distancia:0.5},
		Bioma {tipo:TipoBioma::Tierra, distancia:2.0},
	];
	let perro = Perro::default();
    println!("Hello, world!");
}
