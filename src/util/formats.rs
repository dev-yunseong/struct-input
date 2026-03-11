use std::sync::OnceLock;
use darling::FromMeta;
use regex::Regex;
use quote::{quote, ToTokens};

#[derive(Debug, Clone, FromMeta)]
pub enum Format {
    BaseUrl,
    Name,
    NotAllowWhitespace,
    None,
}

impl ToTokens for Format {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variant_name = format!("{:?}", self);
        let ident = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());

        tokens.extend(quote! {
            Format::#ident
        });
    }
}

impl Format {
    pub fn valid(&self, text: &str) -> bool {
        match self {
            Format::BaseUrl => {
                static RE: OnceLock<Regex> = OnceLock::new();
                RE.get_or_init(||{Regex::new(r"^[a-zA-Z]+://[a-zA-Z0-9.]+(:[0-9]+)?$").unwrap()})
                    .is_match(text)
            },
            Format::Name => {
                static RE: OnceLock<Regex> = OnceLock::new();
                RE.get_or_init(||{Regex::new(r"^[a-zA-Z0-9-]+$").unwrap()})
                    .is_match(text)
            },
            Format::NotAllowWhitespace => {
                !text.chars().any(|c| c.is_whitespace())
            },
            Format::None => true
        }
    }
}