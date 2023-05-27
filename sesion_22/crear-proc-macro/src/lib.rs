use std::path::Path;
use std::fs::File;
use std::io::Read;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use creando_macros::EstaVivo;

// pub trait EstaVivo {
// 	fn check() -> bool;
// }

#[proc_macro_derive(EstaVivo)]
pub fn esta_vivo_derive(input:TokenStream)-> TokenStream {
	let ast = syn::parse(input).unwrap();
	impl_esta_vivo(&ast)//tokestream
}

fn impl_esta_vivo(ast:&syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let gen = quote! {
		impl EstaVivo for #name {
			fn check(&self) -> bool {
				true
			}
		}
	};
	gen.into()
}
