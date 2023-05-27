#[macro_use] 
extern crate colored;
use crate::colored::Colorize;

// seudo funcion
macro_rules! decir_algo {
	// esto se escribe antes de compilar
	// void
	// declaración de inputs
	// arugs vacio
	() => { 
		// rust clasico
		println!("Decir algo!");
	}
}



// macro que crea una función
// metaprogramación -> crear codigo, declarar structs, fn, etc
macro_rules! crear_funcion {
	// toma como argumento el designador 'ident'
	/*
	argumentos: 
	- nombre de la variable -> $nombre
	  a diferencia en rust -> nombre

	- la segunda parte, luego del :, ident
	no es el tipo de rust, sino el tipo para macro
	 */
	($func_name:ident) => {
		fn $func_name() {
			println!(
				"Llamaste la función {:?}()", 
				stringify!($func_name));
		}

	}

}

crear_funcion!(primera);
crear_funcion!(segunda);


/*
Una macro que imprima una expresión de codigo
*/

macro_rules! print_result {
	/*
	En la macro ingresa una expresión de rust y la muestra
	 */
	($expresion:expr) => {
		println!("{:?} = {:?}", stringify!($expresion), $expresion);
	}

}

// una macro que retorne un valor del resultado de la expresión
macro_rules! suma {
	/*
	dos argumentos o valores, que son expr y retorna el valor de la operación
	 */
	($a:expr, $b:expr) => {
		// podemos prepara un template para operar valores
		$a + $b
	}
}

#[derive(Debug, Clone, Default)]
struct Persona {
	nombre: String,
	edad: u8
}

macro_rules! mostrar_metadata {
	($struct:ident)=>{
		println!("{}", stringify!($struct));
		let p = $struct::default();
		println!("Default {:?}", p);	
		println!("Clon {:?}", p.clone());
	}
}

/*
Usando la struct para crear un array y entregarlo como valor
*/

macro_rules! crear_array {
	($struct:ident, $name:expr, $edad:expr) => {
		{
			println!("{}", stringify!($struct));
			let result = [
				$struct {nombre:$name, edad:$edad},
				$struct {nombre:$name, edad:$edad+1},
				$struct {nombre:$name, edad:$edad+2},
				$struct {nombre:$name, edad:$edad+3},
				$struct {nombre:$name, edad:$edad+4},
			];
			println!("Personas: {:?}", &result);
			result
		}
	}
}


/*
Imprimir en colores
*/

macro_rules! imprimir_color_rgb {
	($texto:expr, $r:expr, $g:expr, $b:expr) => {
		println!("{}", format!("{}", $texto).truecolor($r,$g, $b));
	}
}

macro_rules! imprimir_color_hex {
	($texto:expr, $hex:expr) => {
		let hex_value = u32::from_str_radix($hex, 16);
		if let Ok(color) = hex_value {
			let r = ((color>>16) & 0xFF) as u8;
			let g = ((color>>8) & 0xFF) as u8;
			let b = (color & 0xFF) as u8;
			println!("{}", format!("{}", $texto).truecolor(r,g,b));
		} else {
			println!("Error: el valor hexadecimal no es válido");
		}
	};
}




fn main() {
	decir_algo!();

	primera();
	segunda();

	print_result!(1u32 + 1);
	print_result!({
		let x:f32=56.0;
		x*x + 2.0*x - 1.0
	});

	println!("{}",suma!(1,2));
	//suma!(1,2.0);
	println!("{}",suma!(3.0, 5.4));

	mostrar_metadata!(Persona);
	let result = crear_array!(Persona, "José".into(), 15);

	for elem in result.iter() {
		println!("{:?}",elem);
	}

	imprimir_color_rgb!("Esto es un texto en ROJO",255,0,0);
	imprimir_color_rgb!("Esto es un texto en Naranjo",255,80,0);


	imprimir_color_hex!("Esto es un texto en Naranjo usando HEX","ffa500");

}
