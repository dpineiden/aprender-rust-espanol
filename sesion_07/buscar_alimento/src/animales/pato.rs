use crate::base::{self, SerVivo, Nadar, Caminar, Volar, Accion};


#[derive(Clone, Debug)]
pub struct Pato{
	pub nombre: String
}

impl Default for Pato {
	fn default() -> Pato{ Pato { nombre:"Donald".into()}}
}

impl Accion for Pato {
	fn agua(&self)->bool {true}
	fn tierra(&self)->bool {true}
	fn aire(&self)->bool {true}
	fn get_nombre(&self)->&String {&self.nombre}
}

impl Volar for Pato {
	fn avanzar(&self) -> f32 {
		6.8
	}
}

impl Caminar for Pato {
	fn avanzar(&self) -> f32 {
		0.2
	}
}

impl Nadar for Pato {
	fn avanzar(&self) -> f32 {
		8.729
	}
}


impl SerVivo for Pato {
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
