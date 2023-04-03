use std::include_str;

pub fn read_help() {
	let help_text = include_str!("./help.txt");
	println!("{}", help_text);
}
