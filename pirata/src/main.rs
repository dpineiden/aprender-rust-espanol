/*
 *   entradas:
 *  - posicion final
 *  - posicion inicial del objeto
 *  - la 'velocidad' (rapidez con dirección)
 * 
 *  entregar:
 * - un valor real positivo (float)
 * */

mod barco {
    use std::fmt;
    #[derive(Debug,  Clone)]
    pub struct Barco {
        pub name:String, 
        pub position:f32, 
        pub velocidad:f32
        }
    impl Barco {
            pub fn new(name:&str, position:f32, velocidad:f32) -> Self {
                    Barco {name:name.to_string(), position, velocidad}
                }
        }
    impl fmt::Display for Barco {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "Barco {}, posicion: {}, velocidad: {}", self.name, self.position, self.velocidad)
        }
    }
}
use crate::barco::Barco;


fn tiempo_viaje(
    posicion_final:f32, 
    barco:&Barco) -> f32{
        ((posicion_final-barco.position)/barco.velocidad).abs()
    }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tiempo_viaje() {
        let posicion_a: f32 = -30.0;
        let rapidez_a: f32= 5.0;
        let posicion_isla: f32 = -3.0;
        let tiempo_a= 5.4;
        let result =  tiempo_viaje(posicion_isla, posicion_a, rapidez_a);
        assert_eq!(tiempo_a,result);
    }
}


fn main() {
    let barcos = vec![
        Barco::new("A", -15.0, 3.0),
        Barco::new("B", 54.0, 22.0),
        Barco::new("C", -12.0, 0.5),
        Barco::new("D",5.3, 1.1),
        Barco::new("E",-8.3,0.8),
        Barco::new("F",33.3,3.45),
        Barco::new("G",18.0, 2.2),
        Barco::new("H", 23.3, 3.1),
    ];

    let posicion_isla: f32 = -3.0;
    // convertir en iterador -> map -> array o min
    // aplicar tiempo de viaje
    // retornar array con nombre y tiempo de viaje
    // obtener el menor tiempo de viaje
    let result: Vec<(String, f32)> = barcos
        .iter()
        .map(|barco|  {
            let tv = tiempo_viaje(posicion_isla, barco);
            (barco.name.clone(), tv) })
            .collect::<Vec<(String, f32)>>().try_into().unwrap();

    let mut name = "".to_string();
    let mut time = 1000.0;
    for r in result {
        println!("Menor tiempo de viaje {}, {}", r.0, r.1);
        if r.1<time {
            name = r.0;
            time = r.1;
            }
        }
    println!("El barco que llega primero es {} demorando  {} ", name, time);
}


/*
 *     // tupla : (posicion, rapidez)
    // datos de Barco A
    let barco_a = Barco::new( "A", -30.0, 5.0);
    // datos de Barco B
    let barco_b = Barco::new("B",16.0, 7.0);
    
 *     println!("Barco A {}", barco_a);
    println!("Barco B {}", barco_b);
    // posición de isla
    let posicion_isla: f32 = -3.0;
    // necesito calcular el tiempo
    // para llegar a la Isla
    let tiempo_a = tiempo_viaje(posicion_isla, barco_a);
    let tiempo_b = tiempo_viaje(posicion_isla, barco_b);
    if tiempo_a<tiempo_b {
        println!("Llega primero a con {}",tiempo_a);
        } 
    else if tiempo_b<tiempo_a {
        println!("Llega primero b con {}, A demora {}", tiempo_b, tiempo_a);
    } 
    else {
        println!("Ambos barcos llegan al mismo tiempo: {}", tiempo_a);
        }    
 * 
 * */
