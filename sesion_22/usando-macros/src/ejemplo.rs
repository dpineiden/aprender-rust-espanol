//use creando_macros::EstaVivo;
use creando_macros::EstaVivo; // este es el trait
use crear_proc_macro::EstaVivo; // este viene con derive

#[derive(EstaVivo,Debug)]
pub struct Persona {
	nombre: String,
	apellido: String,
	edad: u8
}

impl Persona {
	pub fn esta_vivo(&self) -> bool{
		self.check()
	}
}

struct Roca {
	peso: u32
}

impl EstaVivo for Roca {
	fn check(&self)->bool {
		false
	}
}

// impl EstaVivo for Persona {
// 	pub fn check()->bool {
// 		true
// 	}
// }


pub fn crear_persona(nombre:String, apellido:String, edad:u8) -> Persona {
	Persona{nombre, apellido, edad}
}
