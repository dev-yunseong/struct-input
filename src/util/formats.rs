use std::sync::OnceLock;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum Format {
    BaseUrl,
    Name,
    NotAllowWhitespace,
    None,
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