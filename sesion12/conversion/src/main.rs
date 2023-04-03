fn u8_to_u32_le(array:&[u8]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

// formular bin negativo
fn byte_negativo(val:u8) -> u8 {
	255-val
}

fn show_bytes(value:&i16) {
	let native = value.to_ne_bytes();
	let big_endian = value.to_be_bytes();
	let little_endian = value.to_le_bytes();
	println!("Valor original {}", value);

	println!("Serialización:");
	println!("Native: {:?}", native);
	println!("Big endian: {:?}", big_endian);
	println!("Little endian: {:?}", little_endian);


	// println!("De-Serialización:");
	// let result_native = u8_to_u32_le(&native);
	// let result_be = u8_to_u32_le(&big_endian);
	// let result_le = u8_to_u32_le(&little_endian);
	// println!("Native: {:?}", result_native);
	// println!("Big endian: {:?}", result_be);
	// println!("Little endian: {:?}", result_le);

	println!("Usando u32::from_X_bytes");
	let f_result_native = i16::from_ne_bytes(native.clone());
	let f_result_be = i16::from_be_bytes(native.clone());
	let f_result_le = i16::from_le_bytes(native);

	println!("From Native: {:?}", f_result_native);
	println!("From Big endian: {:?}", f_result_be);
	println!("From Little endian: {:?}", f_result_le);

}


fn main() {

	println!("Max i16 {}", i16::MAX);
	println!("Min i16 {}", i16::MIN);

	let valor_1:i16 = 100;
	let valor_2:i16 = -100;

	println!("Valor positivo:");
	show_bytes(&valor_1);
	println!("Valor negativo:");
	show_bytes(&valor_2);


}
