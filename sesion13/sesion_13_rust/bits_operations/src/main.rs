fn main() {
	// operador carro

	// 00000010
	let basic:u8 = 2;
	println!("Es 1 segunda posición? {}", ((basic<<6)>>7)==1);


	// 10000000
	let solo_uno:u8 = 128;
	println!("Es 1 posición 8? {}", (solo_uno>>7)==1);

	// 11111111
	let todos_uno:u8 = 255;
	println!("Es 255 posición 8? {}", ((todos_uno<<3)>>7)==1);
	
	// 255=11111111
	let ff:u8 = 0xff;
	println!("Es ff posición 8? {}", ((ff<<3)>>7)==1);

}
