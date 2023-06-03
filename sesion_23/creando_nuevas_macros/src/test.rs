#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

macro_rules! deserializar_csv {
    ($csv_file:expr, $struct_name:ty) => {
        deserializar_csv!($csv_file, $struct_name, id);
    };
    ($csv_file:expr, $struct_name:ty, $key_field:ident) => {
        fn deserializar_csv_to_hashmap() -> Result<HashMap<String, $struct_name>, Box<dyn Error>> {
            let mut reader = csv::Reader::from_path($csv_file)?;
            let mut hashmap: HashMap<String, $struct_name> = HashMap::new();

            for result in reader.deserialize() {
                let record: $struct_name = result?;
                let key = {
                    let field_name = stringify!($key_field);
                    if field_name == "id" {
                        record.id.to_string()
                    } else {
                        match record.$key_field {
                            Some(value) => value,
                            None => record.id.to_string(),
                        }
                    }
                };
                hashmap.insert(key, record);
            }

            Ok(hashmap)
        }

        match deserializar_csv_to_hashmap() {
            Ok(hashmap) => {
                println!("Deserialización exitosa: {:?}", hashmap);
            }
            Err(err) => {
                eprintln!("Error al deserializar CSV: {}", err);
            }
        }
    };
}
