use crate::base::{self, SerVivo,Nadar,Caminar,Accion};

#[derive(Clone, Debug)]
pub struct Perro {
	pub nombre:String
}


impl Accion for Perro {
	fn agua(&self)->bool {true}
	fn tierra(&self)->bool {true}
	fn aire(&self)->bool {false}
	fn get_nombre(&self)->&String {&self.nombre}
}

impl Default for Perro {
	fn default() -> Perro {Perro{nombre:"Quiltro".into()}}
}

impl Caminar for Perro {
	fn avanzar(&self) -> f32 {
		6.0
	}
}


impl Nadar for Perro {
	fn avanzar(&self) -> f32 {
		1.0
	}
}

impl SerVivo for Perro {
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
