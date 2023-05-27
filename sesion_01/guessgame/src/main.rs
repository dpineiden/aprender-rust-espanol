use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    println!("=Adivina el número=");
    
    loop {
        println!("Por favor, ingresa tu número");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess
            .trim()
            .parse() {
                    Ok(num) => num,
                    Err(_) => continue
                };
        
        println!("Tu entregaste el siguiente valor {guess}");      
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Demasiado pequeñoll!"),
            Ordering::Greater => println!("Demasiado grande!"),
            Ordering::Equal => {
                println!("Ganaste!!");                   
                break;
                } 
            }  
        
    }
}
