use crate::base::{self, SerVivo, Nadar, Caminar, Accion};

#[derive(Clone, Debug)]
pub struct Humano {
	pub nombre: String,
}

impl Default for Humano {
	fn default() -> Humano{ Humano { nombre:"Pepe".into()}}
}

impl Accion for Humano {
	fn agua(&self)->bool {true}
	fn tierra(&self)->bool {true}
	fn aire(&self)->bool {false}
	fn get_nombre(&self)->&String {&self.nombre}
}

impl Caminar for Humano {
	fn avanzar(&self) -> f32 {
		8.0
	}
}


impl Nadar for Humano {
	fn avanzar(&self) -> f32 {
		1.5
	}
}


impl SerVivo for Humano {
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
