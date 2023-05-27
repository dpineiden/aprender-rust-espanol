use std::path::Path;
use std::fs::File;
use std::io::Read;
use quote::quote;
use syn;

#[macro_export]
macro_rules! vectorcito {
	($( $x:expr ),* ) => {
		{
			let mut temp_vec = Vec::new();
			$(
				temp_vec.push($x);
			)*
			temp_vec	
		}
	};
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}



pub fn crear_ast(path:&Path){
	let mut file = File::open(path)
		.expect("No se puede abrir");
	let mut src = String::new();
	file.read_to_string(&mut src)
		.expect("No se puede leer el archivo");
	//hasta aqui el archivo pasa a ser un texto
	let syntax = syn::parse_file(&src)
		.expect("No se puede parsear a estructura de RUST");
	println!("{:#?}", syntax);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


pub trait EstaVivo {
	//type
	fn check(&self) -> bool;
}

// #[proc_macro_derive(EstaVivo)]
// pub fn esta_vivo_derive(input:TokenStream)-> TokenStream {
// 	let ast = syn::parse(input).unwrap();
// 	impl_esta_vivo(&ast)//tokestream
// }

// fn impl_esta_vivo(ast:&syn::DeriveInput) -> TokenStream {
// 	let name = &ast.ident;
// 	let gen = quote! {
// 		impl EstaVivo for #name {
// 			fn check() -> bool {
// 				true
// 			}
// 		}
// 	};
// 	gen.into()
// }
