// exposición
use super::base;

pub fn show_line(nr:&usize, line:&String, opts: &base::FileOpts) -> Result<(), String>
{
	// hacer match
	let regex = opts.get_regex();
	// field selection
	// la fila que se va a mostrar
	let mut new_line:Vec<String> = vec![];
	
	if opts.get_show_row() {		
		new_line.push(nr.to_string());
	}
	
	if regex.is_match(line) {
		new_line.push(line.into());
		let text = new_line.join("::");
		println!("{}", text);
	}
	// mostrar linea si hay match

	Ok(())
}
