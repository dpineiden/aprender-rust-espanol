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

use tokio::sync::mpsc::Sender;

pub async fn primeros_primos_stream(n:u64, tx:&Sender<u64>)->Vec<u64> {

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
			let new_i = i.clone();
			set_primos.push(i);
			// enviar por el tx el valor
			tx.send(new_i).await.unwrap();
		}
	}
	tx.send(0).await.unwrap();
	set_primos
}



#[cfg(test)]
mod tests {
	use crate::primeros_primos;
    use crate::primeros_primos_stream;
	use tokio::sync::mpsc;

	#[test]
	fn test_primos() {
		let n:u64 = 10;
		let base = vec![2,3,5,7];
		let result = primeros_primos(n);
		assert_eq!(result, base);
	}
	/* tomar un archivo de primos hasta n y comparar*/

	#[tokio::test]
	async fn test_async_primos() {
		let n:u64 = 10;
		// let base = vec![2,3,5,7];
		let (tx, mut rx) = mpsc::channel(1200);
		let result = primeros_primos_stream(n, &tx).await;

		let mut new_vec = vec![];
		
		while let Some(i) = rx.recv().await {
			if i==0 {break; }
			new_vec.push(i);
		}
		assert_eq!(result, new_vec);
	}
	/* tomar un archivo de primos hasta n y comparar*/


}
