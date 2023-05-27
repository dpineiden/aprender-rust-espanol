pub mod perro;
pub mod murcielago;
pub mod humano;
pub mod cuervo;
pub mod gaviota;
pub mod delfin;
pub mod albacora;
pub mod pato;
pub mod gallina;


pub use perro::Perro;
pub use murcielago::Murcielago;
pub use humano::Humano;
pub use cuervo::Cuervo;
pub use gaviota::Gaviota;
pub use delfin::Delfin;
pub use albacora::Albacora;
pub use pato::Pato;
pub use gallina::Gallina;

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
