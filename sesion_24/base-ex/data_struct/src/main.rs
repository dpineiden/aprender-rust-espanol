use crate::colored::Colorize;
macro_rules! diga_algo {
	() => {
		println!("Decir algo!");
	}
}


macro_rules! create_function {
	// esta macro toma un argumento del designator ident
	
	($func_name:ident) => {
		fn $func_name() {
			println!("LLamaste la función {:?}()", stringify!($func_name));
		}
	}
}

create_function!(primera);
create_function!(segunda);

macro_rules! print_result {
	($expression:expr)=>{
		println!("{:?} = {:?}", stringify!($expression), $expression);
	}
}



macro_rules! suma{
 // macth like arm for macro
    ($a:expr,$b:expr)=>{
 // macro expand to this code
        {
// $a and $b will be templated using the value/variable provided to macro
            $a+$b
        }
    }
}

#[derive(Debug)]
struct Persona {
	nombre: String,
	edad: u8
}


macro_rules! mostrar_metadata {
    ($struct_name:ident) => {
        fn main() {
            println!("Metadata de la estructura: {:?}", stringify!($struct_name));
            //println!("Debug: {:?}", $struct_name::debug());
            // println!("Clone: {:?}", $struct_name::clone());
            // println!("PartialEq: {:?}", $struct_name::partial_eq());
            // println!("Eq: {:?}", $struct_name::eq());
            // println!("PartialOrd: {:?}", $struct_name::partial_ord());
            // println!("Ord: {:?}", $struct_name::ord());
            // println!("Default: {:?}", $struct_name::default());
        }
    };
}
macro_rules! informacion {
    ($struct:ident, $name:expr, $edad:expr)=>{{
		println!("{:?}", stringify!($struct));
		let result = [
			$struct {nombre:$name, edad:$edad},
			$struct {nombre:$name, edad:$edad+1},
			$struct {nombre:$name, edad:$edad+2},
		];
		println!("Personas: {:?}", &result);
		result
    }
	}
}

#[macro_use]
extern crate colored;

macro_rules! imprimir_color_rgb {
    ($texto:expr, $r:expr, $g:expr, $b:expr) => {
        println!("{}",
            format!("{}", $texto).truecolor($r, $g, $b)
        );
    };
}


macro_rules! imprimir_color_hex {
    ($texto:expr, $hex:expr) => {
        let hex_value = u32::from_str_radix($hex, 16);
        if let Ok(color) = hex_value {
            let r = ((color >> 16) & 0xFF) as u8;
            let g = ((color >> 8) & 0xFF) as u8;
            let b = (color & 0xFF) as u8;

            println!("{}", format!("{}", $texto).truecolor(r, g, b));
        } else {
            println!("Error: El valor hexadecimal no es válido.");
        }
    };
}
/*
struct Layer<const I: usize, const O: usize>;

macro_rules! example {
    // Do something interesting for a given pair of arguments
    ($a:literal, $b:literal) => {
        Layer::<$a, $b>;
    };

    // Recursively traverse the arguments
    ($a:literal, $b:literal, $($rest:literal),+) => {
        example!($a, $b);
        example!($b, $($rest),*);
    };
}

fn main() {
    example!(1, 2, 3);
}

*/
/*
elementos para macros como variables expresivas
https://doc.rust-lang.org/reference/macros-by-example.html

item :: item object
block :: expresion en bloque
stmt :: un Statement si el semicolon
pat_param :: pattern no top alt
pat :: similar
expr :: expresion
ty :: type
ident :: IDENTIFIER_OR_KEYWORD
path :: TypePath
tt :: TokenTree
meta:: un Attr
lifetime :: un token LIFETIME
vis :: selector de visibilidad
literal :: maychaes LiteralExpression


reglas de uso:


    expr and stmt may only be followed by one of: =>, ,, or ;.
    pat_param may only be followed by one of: =>, ,, =, |, if, or in.
    pat may only be followed by one of: =>, ,, =, if, or in.
    path and ty may only be followed by one of: =>, ,, =, |, ;, :, >, >>, [, {, as, where, or a macro variable of block fragment specifier.
    vis may only be followed by one of: ,, an identifier other than a non-raw priv, any token that can begin a type, or a metavariable with a ident, ty, or path fragment specifier.
    All other fragment specifiers have no restrictions.

*/

fn main() {

	let valor1:u32 = 34;
	let bytes1 = valor1.to_be_bytes();
	let valor2:f32 = 34.0;
	let bytes2 = valor2.to_be_bytes();
	let string = "Hola taller de rust";
	let bytes3  = string.as_bytes();
	let struct_format = format!(">If{}s", bytes3.len());
	let all_bytes = [&bytes1,&bytes2,bytes3].concat();
	println!("{:?}",struct_format);
	println!("Bytes array: {:?}",&all_bytes);
	println!("---");
    println!("Hello, world!");
	diga_algo!();
	// test structure
	//let all_bytes2 = s.pack(valor1, valor2, string);
	//println!("Bytes array: {:?}",&all_bytes2);
	primera();
	segunda();
	// mostrar expresion

	print_result!(1u32+1);

	print_result!({
		let x = 1u32;
		x * x + 2 * x - 1
	});

	println!("{:?}",suma!(2,3));

	let res=informacion!(Persona, "Jose".into(), 12);
	println!("{:?}", res);

	mostrar_metadata!(Persona);

	imprimir_color_rgb!("Hola taller de rust",30,200,50);
}
