struct Persona {
	pub nombre:String,
	pub edad: u32
}

fn main() {
	let persona = Persona {nombre:"Jose".to_string(), edad:32};
	let potencia = 3;
	let lista = vec![2,5,7,9,11];
	let elevar =  move |c:u32|->u32 {u32::pow(c,potencia) + persona.edad};
	let results:Vec<u32> = lista.into_iter().map(elevar).collect();
	println!("{:?}", results);

	// let suma =  |a:u32, b:u32| -> u32{
	// 	let resultado = a + b;
	// 	resultado
	// };
	// let res = suma(12,34);
	// println!("El resultado de la suma es : {}", res);
}
