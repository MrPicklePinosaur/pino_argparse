extern crate proc_macro

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Table)]
pub fn derive_table(input: TokenStream) -> TokenStream {

    let ast = syn::parse(input).unwrap();

    impl_table(&ast)
}

fn impl_table(ast: &syn::DeriveInput) -> TokenStream {

}
