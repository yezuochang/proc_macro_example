extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro]
pub fn testmacro(input: TokenStream) -> TokenStream {
    return input;
}
