fn u8_to_u32_le(array:&[u8]) -> u32 {
    ((array[0] as u32) << 24) |
    ((array[1] as u32) << 16) |
    ((array[2] as u32) <<  8) |
    ((array[3] as u32) <<  0)
}

fn u8_to_i32_le(array:&[u8]) -> i32 {
    ((array[0] as i32) << 24) |
    ((array[1] as i32) << 16) |
    ((array[2] as i32) <<  8) |
    ((array[3] as i32) <<  0)
}

fn check(number:&u8) -> bool {
	const SIGNED:u8 = 128;//10000000
	(number & SIGNED) == SIGNED
}


fn show_bytes_i32(value:&i32) {
	let native = value.to_ne_bytes();

	for number in native.iter() {
		println!("Is negative :: {:?} -> {}", number, check(number))
	}

	let negative = native.iter().all(|n| check(n));
	println!("La serie es negativa? :{}", negative);

	let big_endian = value.to_be_bytes();
	let little_endian = value.to_le_bytes();
	println!("Valor original {}", value);

	println!("Serialización:");
	println!("Native: {:?}", native);
	println!("Big endian: {:?}", big_endian);
	println!("Little endian: {:?}", little_endian);


	// println!("De-Serialización:");
	let result_native = u8_to_i32_le(&native);
	let result_be = u8_to_i32_le(&big_endian);
	let result_le = u8_to_i32_le(&little_endian);
	println!("Native: {:?}", result_native);
	println!("Big endian: {:?}", result_be);
	println!("Little endian: {:?}", result_le);

	println!("Usando u32::from_X_bytes");
	let f_result_native = i32::from_ne_bytes(native);
	let f_result_be = i32::from_be_bytes(native);
	let f_result_le = i32::from_le_bytes(native);

	println!("From Native: {:?}", f_result_native);
	println!("From Big endian: {:?}", f_result_be);
	println!("From Little endian: {:?}", f_result_le);

}

fn show_bytes_u32(value:&u32) {
	let native = value.to_ne_bytes();


	for number in native.iter() {
		println!("Is negative :: {:?} -> {}", number, check(number))
	}

	let negative = native.iter().all(|n| check(n));
	println!("La serie es negativa? :{}", negative);
	let big_endian = value.to_be_bytes();
	let little_endian = value.to_le_bytes();
	println!("Valor original {}", value);

	println!("Serialización:");
	println!("Native: {:?}", native);
	println!("Big endian: {:?}", big_endian);
	println!("Little endian: {:?}", little_endian);


	// println!("De-Serialización:");
	let result_native = u8_to_u32_le(&native);
	let result_be = u8_to_u32_le(&big_endian);
	let result_le = u8_to_u32_le(&little_endian);
	println!("Native: {:?}", result_native);
	println!("Big endian: {:?}", result_be);
	println!("Little endian: {:?}", result_le);

	println!("Usando u32::from_X_bytes");
	let f_result_native = u32::from_ne_bytes(native);
	let f_result_be = u32::from_be_bytes(native);
	let f_result_le = u32::from_le_bytes(native);

	println!("From Native: {:?}", f_result_native);
	println!("From Big endian: {:?}", f_result_be);
	println!("From Little endian: {:?}", f_result_le);
}


fn main() {

	println!("Max i32 {}", i32::MAX);
	println!("Min i32 {}", i32::MIN);

	println!("Max u32 {}", u32::MAX);
	println!("Min u32 {}", u32::MIN);


	let u_valor_1:u32 = 100;
	//let u_valor_2:u32 = -100; --> error en el chqueo <0

	let s_valor_1:i32 = 56;
	let s_valor_2:i32 = -56;

	println!("Valor unsigned:");
	show_bytes_u32(&u_valor_1);
	println!("=================");
	println!("SIGNED VALUES i32");
	println!("=================");
	println!("Valor signed positivo:");
	show_bytes_i32(&s_valor_1);
	println!("Valor signed negativo:");
	show_bytes_i32(&s_valor_2);

}
