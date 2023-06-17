pub fn primeros_primos(n:u64)->Vec<u64> {

	let mut set_primos = vec![];

	for i in 2..n {
		let mut bandera = true;
		for j in set_primos.iter() {
			if i % *j == 0 {
				bandera = false;
				break;
			} 
		}
		if bandera {
			set_primos.push(i);
		}
	}
	set_primos
}


#[cfg(test)]
mod tests {
	use crate::primeros_primos;
	#[test]
	fn test_primos() {
		let n:u64 = 10;
		let base = vec![2,3,5,7];
		let result = primeros_primos(n);
		assert_eq!(result, base);
	}
	/* tomar un archivo de primos hasta n y comparar*/

}
