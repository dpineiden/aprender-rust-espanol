use crate::base::{self, SerVivo, Accion, Caminar};

#[derive(Default)]
pub struct Gallina{
	patas: u8,
	pelaje: bool,
	cola: bool,
	alimentacion: String
}


impl Accion for Gallina {
	fn agua(&self)->bool {false}
	fn tierra(&self)->bool {true}
	fn aire(&self)->bool {false}
}


impl Caminar for Gallina {
	fn avanzar(&self) -> f32 {
		0.6
	}
}


impl SerVivo for Gallina {
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
