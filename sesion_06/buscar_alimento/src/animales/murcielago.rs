use crate::base::{self, SerVivo,Volar, Accion};


#[derive(Default)]
pub struct Murcielago{
	patas: u8,
	pelaje: bool,
	cola: bool,
	alimentacion: String
}

impl Accion for Murcielago 
{
	fn agua(&self)->bool {false}
	fn tierra(&self)->bool {false}
	fn aire(&self)->bool {true}
}

impl Volar for Murcielago {
	fn avanzar(&self) -> f32 {
		3.0
	}
}


impl SerVivo for Murcielago  {
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
