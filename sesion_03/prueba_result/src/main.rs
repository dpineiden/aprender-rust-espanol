fn positivo(valor:i64)->Result<i64, &'static str> {
	if valor >= 0 {
		return Ok(valor)
	} 
	Err("No es positivo")
}



fn main() {
    println!("Hello, world!");
	let x: Result<i32, &str> = Ok(10);
	println!("Is ok? {}",x.is_ok());
	let valor1:i64 = 100;
	let valor2:i64 = -30;

	let result = positivo(valor1).unwrap();

	println!("Resultado {}", result);

	// let result2 = positivo(valor2).unwrap();

	// println!("Resultado {}", result2);

	match positivo(valor2) {
		Ok(valor) => println!("Valor ok {}", valor),
		Err(err) => eprintln!("Error {}", err)	
	}

}
