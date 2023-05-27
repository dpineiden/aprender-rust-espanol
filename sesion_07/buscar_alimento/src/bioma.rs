use super::base::Accion;

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
	pub fn check(&self, animal:&dyn Accion) -> bool 
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
