use std::path::PathBuf;
use std::collections::HashMap;
use derivative::Derivative;
use regex::RegexSet;

#[derive(Debug)]
pub struct FileOpts {
	estricto: bool,
	show_row:bool,	
	re: RegexSet,
}

impl FileOpts {
	fn new(estricto: bool, show_row: bool, re: RegexSet) -> Self {
		Self {estricto, show_row, re}
	}

	pub fn build(opts:HashMap<String,String>) -> Self {
		let show_row = opts.get("-R").is_some();
		let estricto = match opts.get("-E") {
			Some(_)=>{	
				let re = RegexSet::new([r"^\d+$",r"^\d{1,3}(_\d{3})+$"]).unwrap();
				Self::new(true, show_row, re)
			},
			None => {
				let re = RegexSet::new([r"\d+",r"\d{1,3}(_\d{3})+"]).unwrap();
				Self::new(false, show_row, re)
			}
		};
		estricto
	}

	pub fn get_estricto(&self)->bool{
		self.estricto
	}

	pub fn get_show_row(&self)->bool{
		self.show_row
	}

	pub fn get_regex(&self)->&RegexSet {
		&self.re
	}

}

// crear enum input : stdin/pathbuf
#[derive(Debug, Derivative, Eq, PartialEq)]
#[derivative(Hash)]
pub enum TextInput {
	FilePath(PathBuf),
	StdIn
}

