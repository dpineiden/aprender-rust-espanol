extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn timing(_: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function item
    let input = parse_macro_input!(item as ItemFn);

    // Extract the name of the function
    let fn_name = input.sig.ident.clone();

    // Generate a new identifier for the timer variable
    let timer_ident = Ident::new(&format!("__timer_{}", fn_name), Span::call_site());

    // Generate the expanded code
    let expanded = quote! {
        #[allow(unused_variables)]
        #[allow(unused_assignments)]
        #[allow(unused_mut)]
        #[allow(non_snake_case)]
        #input

        fn #fn_name() {
            let #timer_ident = std::time::Instant::now();
            #fn_name();
            let elapsed = #timer_ident.elapsed();
            println!("{} executed in {:?}.", stringify!(#fn_name), elapsed);
        }
    };

    TokenStream::from(expanded)
}
