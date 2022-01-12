#![feature(proc_macro_span)]
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro]
pub fn testmacro(input: TokenStream) -> TokenStream {
    let _caller_path = input
        .clone()
        .into_iter()
        .last()
        .unwrap()
        .span()
        .source_file()
        .path();
    return input;
}
