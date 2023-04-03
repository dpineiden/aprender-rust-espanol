pub enum Nutricion {
	Autotrofa,
	Heterotrofa
}

pub enum TipologiaCelular{
	Eucariota,
	Procariota
}


pub enum OrganizacionCelular {
	Unicelular,
	Pluricelular
}

pub enum Respiracion {
	Aerobica,
	Anaerobica
}

pub enum Reproduccion {
	Sexual,
	Asexual,
	Esporas
}

pub enum Locomocion {
	Automovil,
	Inmovil
}

pub trait SerVivo {
	// nutricion
	fn nutricion(&self) -> Nutricion;
	// tipologia_celular
	fn tipologia_celular(&self) -> TipologiaCelular;
	// org_celular
	fn organizacion_celular(&self)-> OrganizacionCelular;
	// respiracion
	fn respiracion(&self)->Respiracion;
	// reproduccion 
	fn reproduccion(&self) -> Reproduccion;
	// locomocion
	fn locomocion(&self)-> Locomocion;
}


/*
Reino de los seres vivos
*/
pub enum Reino <S> where S:SerVivo{
	Animal(S),
	Vegetal(S),
	Fungi(S),
	Prototista(S),
	Monera(S)
}


pub trait Accion {
	fn agua(&self)->bool;
	fn tierra(&self)->bool;
	fn aire(&self)->bool;
}

pub trait Nadar: Accion {
	fn avanzar(&self)-> f32;
}

pub trait Caminar: Accion {
	fn avanzar(&self)-> f32;
}

pub trait Volar: Accion {
	fn avanzar(&self)-> f32;
}


pub fn avanzar_en_agua<T:SerVivo+Nadar>(animal:&T, distancia: f32) -> f32 {
	let rapidez = Nadar::avanzar(animal);
	distancia  / rapidez
}

pub fn avanzar_en_tierra<T:SerVivo+Caminar>(animal:&T, distancia: f32) -> f32 {
	let rapidez = Caminar::avanzar(animal);
	distancia  / rapidez
}

pub fn avanzar_en_aire<T:SerVivo+Volar>(animal:&T, distancia: f32) -> f32 {
	let rapidez = Volar::avanzar(animal);
	distancia  / rapidez
}

