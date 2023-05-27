// nombre, posicion, rapidez
//type Barco<'a> = (&'a str, f32, f32);

mod barco {
    use std::fmt;
    
    pub struct Barco {
        pub name: String,
        pub position: f32,
        pub rapidez: f32
        }
    impl Barco {
        pub fn new<'a>(name:&str, position: f32, rapidez: f32)  -> Self {
                Barco {name:name.to_string(), position, rapidez}
            }
        }
    impl fmt::Display for Barco  {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Barco {}, posicion {}, rapidez {}", self.name, self.position, self.rapidez)
                }
        }
    }

use crate::barco::Barco;

fn tiempo_viaje(posicion_isla:f32, barco:&Barco)->f32 {
            let diff = posicion_isla-barco.position;               
            let dir = (diff).abs()/(diff); 
            let tiempo = (diff)/(dir*barco.rapidez);        
            tiempo
}

#[cfg(test)]
mod tests {
      use super::*;

    #[test]
    fn test_tiempo_viaje() {
        let posicion_isla: f32 = -3.0;
        let posicion_a: f32 = -30.0;
        let rapidez_a: f32= 5.0;
        let barco = Barco::new("Test", posicion_a, rapidez_a);
        let tiempo_a= 5.4;
        let result =  tiempo_viaje(posicion_isla, &barco);
        assert_eq!(tiempo_a,result);
    }  
   
}

fn main() {
    // isla del tesoro
    let posicion_isla:f32 = -3.3; // km;    
    // array de barcos
    let barcos = vec![
       Barco::new("A", -15.0, 3.0),
       Barco::new ("B", 54.0, 22.0),
       Barco::new ("C", -12.5, 0.5),
       Barco::new("D", -8.3, 4.2),
       Barco::new("E", 3.1, 2.1),
       Barco::new("F", 30.3, 12.5)
    ];
    
    let result: Vec<(String, f32)> = barcos
        .iter()
        .map(|barco| {
            let tiempo = tiempo_viaje(posicion_isla, &barco);        
            (barco.name.clone(), tiempo)
        }).collect::<Vec<(String, f32)>>().try_into().unwrap();

    let mut mejor_barco = "".to_string();
    let mut mejor_tiempo:f32 = 100000000.0;

    for r in result {
        println!("Menor tiempo de viaje {}, {}",r.0, r.1);
        if r.1 < mejor_tiempo {
                mejor_barco= r.0;
                mejor_tiempo=r.1;
            }
        }

    println!("El barco que llega primero es  {}, demorando {} hr", mejor_barco, mejor_tiempo);
    /*
    
    let mut mejor_tiempo:f32 = 0.0;
    let mut mejor_barco : &str = "";
    let mut flag:bool =  true;
    
    for (i,barco) in barcos.into_iter().enumerate() {
        println!("index {},Barco {}, posicion {}, rapidez {}", i, barco.0, barco.1, barco.2);
        //diferencia
        let diff = posicion_isla-barco.1;               
        // direccion
         let dir = (diff).abs()/(diff); 
        // el tiempo de viaje
        let tiempo = (diff)/(dir*barco.2);        
        if flag {
            mejor_tiempo = tiempo;
            mejor_barco  =barco.0;
            flag = false;
            }
        if tiempo < mejor_tiempo {
            mejor_tiempo = tiempo;
            mejor_barco = barco.0;
            }
        // evaluar el menor tiempo de viaje
        //tiempos.push(t_a);
    }
    println!("El mejor barco es {}, llega primero con  {} hr", mejor_barco, mejor_tiempo);
    // calcular el mínimo
    //let min_value = tiempos.iter().min();
    //match min_value {
    //    Some(minimo)=> println!("El valor minimo es {}", minimo),
     //   None => println!("El vector de tiempos está vacio")
     //   }
    // barco a
    let posicion_a:f32 = -25.3 ;// km
    let rapidez_a:f32 = 5.5; // km/hr
    // barco b
    let posicion_b:f32 = 19.65 ;// km
    let rapidez_b:f32 = 6.7; // km/hr
    // direccion
    let diff_a = posicion_isla-posicion_a;
    let diff_b = posicion_isla-posicion_b;
    // direccion : +1 o -1
    let dir_a = (diff_a).abs()/(diff_a);
    let dir_b = (diff_b).abs()/(diff_b);
    println!("A se dirige a la {}", dir_a);
    println!("B se dirige a la {}", dir_b);
    let t_a = (diff_a)/(dir_a*rapidez_a);
    let t_b = (diff_b)/(dir_b*rapidez_b);
    println!("Tiempo de A {}", t_a);
    println!("Tiempo de B {}", t_b);
    if t_a<t_b {
        println!("El barco A demora menos y llega a la isla primero, {} hr", t_a);
        }
    else if t_b<t_a {
        println!("El barco B demora menos y llega a la isla primero, {} hr", t_b);
        } else {
            println!("Ambos barcos demoran lo mismo, {} hr == {} hr", t_a, t_b);
    }
    */
}
