pub mod perro;
pub mod murcielago;
pub mod humano;
pub mod cuervo;
pub mod gaviota;
pub mod delfin;
pub mod albacora;
pub mod pato;
pub mod gallina;

use crate::base::{SerVivo, Accion};


pub enum Animal <S:SerVivo+Accion> {
	Perro(S),
	Murcielago(S),
	Humano(S),
	Cuervo(S),
	Gaviota(S),
	Delfin(S),
	Albacora(S),
	Pato(S),
	Gallina(S),
}
