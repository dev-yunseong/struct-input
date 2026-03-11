mod attr;

use proc_macro::TokenStream;
use darling::{FromDeriveInput, FromField, FromMeta};
use darling::export::syn::{self, Data, DeriveInput, Field, Fields, Token};
use darling::export::syn::punctuated::Punctuated;
use quote::quote;
use attr::{FieldArgs, StructArgs};

pub fn impl_struct_input(ast: &DeriveInput) -> TokenStream {

    let name = &ast.ident;

    let fields = get_field(&ast.data);
    let struct_args = StructArgs::from_derive_input(ast).unwrap();
    let fields_args = fields.iter()
        .map(|f| FieldArgs::from_field(f).unwrap())
        .collect::<Vec<_>>();

    let field_assignments = fields_args.iter().map(|f| {
        let name = &f.ident;
        let format = &f.format;
        let format = syn::Ident::from_string(format.as_ref().unwrap()).unwrap();
        let prompt = name.as_ref().unwrap().to_string();

        quote! {
            #name: ::struct_input::read_string(#prompt, ::struct_input::Format::#format).await
        }
    });

    let result = quote! {
        impl ::struct_input::StructInputTrait for #name {
            async fn from_input() -> Self {
                Self {
                    #( #field_assignments ),*
                }
            }
        }
    };

    result.into()
}


fn get_field(data: &Data) -> &Punctuated<Field, Token![,]> {
    if let Data::Struct(syn::DataStruct {
         fields: Fields::Named(syn::FieldsNamed { named, .. }),
         ..
     }) = data {
        named
    } else {
        panic!("This macro can only be used on structs with named fields.");
    }
}