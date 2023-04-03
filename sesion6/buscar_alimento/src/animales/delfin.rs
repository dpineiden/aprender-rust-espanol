use crate::base::{self, SerVivo, Nadar, Accion};


#[derive(Default)]
pub struct Delfin{
	patas: u8,
	pelaje: bool,
	cola: bool,
	alimentacion: String
}

impl Accion for Delfin {
	fn agua(&self)->bool {true}
	fn tierra(&self)->bool {false}
	fn aire(&self)->bool {false}
}


impl Nadar for Delfin {
	fn avanzar(&self) -> f32 {
		8.729
	}
}


impl SerVivo for Delfin {
	// nutricion
	fn nutricion(&self) -> base::Nutricion {
		base::Nutricion::Heterotrofa
	}
	// tipologia_celular
	fn tipologia_celular(&self) -> base::TipologiaCelular {
		base::TipologiaCelular::Eucariota
	}
	// org_celular
	fn organizacion_celular(&self)-> base::OrganizacionCelular {
		base::OrganizacionCelular::Pluricelular
	}
	// respiracion
	fn respiracion(&self)->base::Respiracion {
		base::Respiracion::Aerobica
	}
	// reproduccion 
	fn reproduccion(&self) -> base::Reproduccion {
		base::Reproduccion::Sexual
	}
	// locomocion
	fn locomocion(&self)-> base::Locomocion {
		base::Locomocion::Automovil
	}

}
