fn formula(bytes:[u8;8]) -> f64 {
	let bits = u64::from_ne_bytes(bytes);
	f64::from_bits(bits)
	//println!("Bits {}", bits);
	//println!("val {}", val);
	//val
}


fn bit_position(val:&u64,index:usize)->f64{
	((val<<(12+index))>>63) as f64
}

fn formula_step(bytes:[u8;8]) -> f64 {
	//https://www.rfwireless-world.com/Tutorials/floating-point-tutorial.html
	// /https://www.rfwireless-world.com/Tutorials/floating-point-tutorial.html
	//s : signed
	//m : matissa
	//e: format single precision : 8
	//bias: 2^(e-1) - 1
	// valor : (-1)^s * 1.m x 2 ^(e-bias)
	let bits = u64::from_ne_bytes(bytes);
	let val = f64::from_bits(bits);

	println!("Proceso array->bits->float64 {}", val);
	//println!("Bits {}", bits);
	//println!("val {}", val);

	let signo = bits>>63;
	//let signo_b = bit_position(&bits, 64) as usize;
	println!("Signo {}", signo);
	// //assert_eq!(signo_a, signo_b);

	let check_exponente = (52..=62)
		.fold(0u64,|acc, v| {
			acc + 2_u64.pow(v as u32)
		});

	let exponente = (bits & check_exponente)>>52;
	let exponente_b = (bits<<1)>>53;
	// // por operación & y por desplazamiento ok
	println!("Check exponentes-> a:{} b: {}", exponente, exponente_b);
	assert_eq!(exponente, exponente_b);

	let exp_value = u64::pow(2, (exponente-1023) as u32) as f64;

	//let mantisa = (bits << 12) >> 12;

	//assert_eq!(mantisa, bits);

	let mantisa_val = 1.0 + (0..=51)
									   .fold(
										   0.0, 
										   |acc, v|{
											   acc +
												   bit_position(&bits,v
																as
																usize)*(
													   f64::powi(2.0,-(v+1)) )
														 });
	println!("Mantisa val {}", mantisa_val);
	(i8::pow(-1, signo as u32) as f64) * mantisa_val * exp_value
	//0.0
}

fn main() {
	let number = 13.45;
	let neg_number = -13.45;

	println!("Number: {:?}", number);
	println!("Negative Number: {:?}", neg_number);

	let number_bytes = f64::to_ne_bytes(number);
	let neg_number_bytes = f64::to_ne_bytes(neg_number);

	// println!("Number in bytes: {:?}", number_bytes);
	// println!("Negative Number in bytes: {:?}", neg_number_bytes);

	// // la reconversión a f64 sería usando el método:
	// let rec_number = f64::from_ne_bytes(number_bytes);
	// let rec_neg_number = f64::from_ne_bytes(neg_number_bytes);

	// println!("Recovered Number: {:?}", rec_number);
	// println!("Recovered  Negative Number: {:?}", rec_neg_number);


	// let valor = formula(number_bytes);
	// println!("Usinf formula Number f64: {:?}", valor);

	// let neg_valor = formula(neg_number_bytes);
	// println!("Usinf formula Number f64: {:?}", neg_valor);
		
	let val = formula_step(neg_number_bytes);
	println!("Fórmula manual {}", val);
}
