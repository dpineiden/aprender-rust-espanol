use std::thread;
use std::time::Duration;

fn main() {
	thread::spawn(||{
		for i in 1..10 {
			println!("Hola, el número {} del hilo activado", i);
			thread::sleep(
				Duration::from_millis(100)
			);
		}
	});

	for i in 1..5 {
		println!("Hola, el número {} del hilo principal", i);
			thread::sleep(
				Duration::from_millis(150)
			);
	}
}
