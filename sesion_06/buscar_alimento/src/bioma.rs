use super::base::{self, SerVivo, Accion, Nadar, Caminar, Volar};
use crate::animales::Animal;

pub enum TipoBioma {
	Agua,
	Tierra,
	Aire
}

pub struct Bioma {
	pub tipo: TipoBioma,
	pub distancia: f32
}


impl Bioma {
	/*
	Chequear self.tipo trait accion con animal
	 */
	pub fn check<S:SerVivo+Accion>(self:&Self, animal:S) -> bool 
	{
		match self.tipo {
			TipoBioma::Agua => {
				animal.agua()
			},
			TipoBioma::Tierra => {
				animal.tierra() 
			},
			TipoBioma::Aire => {
				animal.aire()
			},			
		}
	}

}
