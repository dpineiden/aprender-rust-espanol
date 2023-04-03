use crate::base::{self, SerVivo, Nadar, Accion};


#[derive(Clone, Debug)]
pub struct Albacora{
	pub nombre: String
}


impl Default for Albacora {
	fn default() -> Albacora { Albacora{nombre:"Pescadote".into()}}
}

impl Accion for Albacora {
	fn agua(&self)->bool {true}
	fn tierra(&self)->bool {false}
	fn aire(&self)->bool {false}
	fn get_nombre(&self)->&String {&self.nombre}

}

impl Nadar for Albacora {
	fn avanzar(&self) -> f32 {
		8.729
	}
}


impl SerVivo for Albacora {
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
