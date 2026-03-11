use proc_macro::TokenStream;
use crate::derive::impl_struct_input;
use darling::export::syn;

mod derive;

#[proc_macro_derive(StructInput, attributes(struct_input))]
pub fn struct_input_drive(input: TokenStream) -> TokenStream {

    let ast = syn::parse(input).unwrap();

    impl_struct_input(&ast)
}

