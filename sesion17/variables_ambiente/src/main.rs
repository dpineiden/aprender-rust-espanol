use std::env;
use globalenv::{set_var, unset_var};
use dotenv::dotenv;

fn main() {
	dotenv().ok();
	
    // println!("Hello, world!");
	// let variable = "VARIABLE";
	// let val = env::var(variable).unwrap();
	// println!("{}",format!("Variable : {variable} -> {val}"));
	// let edad = 39;
	// set_var("EDAD", &format!("{edad}"));

	// let variable = "EDAD";
	// let val = env::var(variable).unwrap();
	// println!("{}",format!("EDAD : {variable} -> {val}"));

	let edad = env::var("EDAD").unwrap();
	let ciudad = env::var("CIUDAD").unwrap();
	let profesion = env::var("PROFESION").unwrap();

	println!("{}:{}:{}", edad,ciudad,profesion);
	
}
