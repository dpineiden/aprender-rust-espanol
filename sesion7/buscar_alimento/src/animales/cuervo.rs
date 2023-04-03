use crate::base::{self, SerVivo,Volar, Caminar, Accion};


#[derive(Clone, Debug)]
pub struct Cuervo{
	pub nombre:String
}

impl Default for Cuervo {
	fn default() -> Cuervo{ Cuervo { nombre:"Alan Poe".into()}}
}

impl Accion for Cuervo {
	fn agua(&self)->bool {false}
	fn tierra(&self)->bool {true}
	fn aire(&self)->bool {true}
	fn get_nombre(&self)->&String {&self.nombre}
}

impl Volar for Cuervo {
	fn avanzar(&self) -> f32 {
		6.8
	}
}

impl Caminar for Cuervo {
	fn avanzar(&self) -> f32 {
		0.1
	}
}


impl SerVivo for Cuervo  {
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
