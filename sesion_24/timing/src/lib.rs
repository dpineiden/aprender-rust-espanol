extern crate proc_macro;


use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn};


#[proc_macro_attribute]
pub fn medir_timing(attr:TokenStream, item:TokenStream) -> TokenStream {
	// // parse al item fn
	// let input = parse_macro_input!(item as ItemFn);
	// // extraer el nombre de la fn
	// let fn_name = input.sig.ident.clone();

	// // genera un nuevo identificador para el timer

	// let timer_ident = Ident::new(
	// 	&format!("__timer__{}", fn_name),
	// 	Span::call_site());

	// // modo macro
	// let expanded  = quote! {
	// 	#[allow(unused_variables)]
	// 	#[allow(unused_assignments)]
	// 	#[allow(unused_mut)]
	// 	#[allow(non_snake_case)]
	// 	#input

	// 	fn #fn_name() {
	// 		let #timer_ident = std::time:Instant::now();
	// 		#fn_name();
	// 		let elapsed = #timer_ident.elapsed();
	// 		println!("{} executed in {:?}", 
	// 				 stringify(#fn_name), 
	// 				 elapsed);
	// 	}
	// };
	// TokenStream::from(expanded)
	println!("----------------------");
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
	item
}
