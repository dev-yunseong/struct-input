use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Type;
use crate::derive::attr::FieldArgs;

pub fn field_to_quote(field_args: &FieldArgs) -> TokenStream {
    let name = &field_args.ident;

    let format = extract_format(&field_args.format);
    let prompt = extract_prompt(&field_args);

    match get_type_name(&field_args.ty).as_deref() {
        Some("Option") => quote! {#name: ::struct_input::read_string_option(#prompt, ::struct_input::Format::#format).await},
        Some("String") => {
            let default = extract_default(&field_args.default);
            quote! {
                #name: ::struct_input::read_string(#prompt, ::struct_input::Format::#format, #default).await
            }
        },
        Some("i32") => quote! {#name: ::struct_input::read_int(#prompt).await},
        _ => panic!(""),
    }
}

fn get_type_name(ty: &Type) -> Option<String> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return Some(segment.ident.to_string());
        }
    }
    None
}

fn extract_prompt(field_args: &FieldArgs) -> String {
    let message = &field_args.message;

    if let Some(message) = message {
        message.to_string()
    } else {
        let name = &field_args.ident.as_ref();
        let name = name.unwrap().to_string();
        format!("--- type {name} ---")
    }
}

fn extract_format(format: &Option<String>) -> Ident {
    Ident::from_string(
        format.as_ref().unwrap_or(&String::from("None"))
    ).unwrap()
}

fn extract_default(default: &Option<String>) -> TokenStream {
     if let Some(default) = default {
         quote!(Some(String::from(#default)))
     } else {
         quote!(None)
     }
}