use darling::{FromField, FromDeriveInput};
use darling::export::syn::Ident;

#[derive(FromDeriveInput)]
#[darling(attributes(struct_input))]
pub struct StructArgs {
    #[darling(default)]
    pub title: Option<String>,
}

#[derive(FromField)]
#[darling(attributes(struct_input))]
pub struct FieldArgs {
    pub ident: Option<Ident>,
    #[darling(default)]
    pub format: Option<String>,
    #[darling(default)]
    pub message: Option<String>,
    #[darling(default)]
    pub default: Option<String>,
}
