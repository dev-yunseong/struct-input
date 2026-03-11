pub mod formats;

use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::Format;

pub async fn read_string_option(prompt: &str, format_checker: Format) -> Option<String> {
    let mut reader = BufReader::new(io::stdin());
    loop {
        println!("{prompt}");
        println!("(Leave empty and press Enter to set as None)");
        let mut value = String::new();
        match reader.read_line(&mut value).await {
            Ok(_) => (),
            Err(_) => continue
        }

        let value = value.trim();

        if value.is_empty() {
            return None
        }

        if format_checker.valid(value) {
            return Some(value.to_string());
        } else {
            println!("invalid input");
        }
    }
}

pub async fn read_string(prompt: &str, format_checker: Format, default: Option<String>) -> String {
    let mut reader = BufReader::new(io::stdin());
    loop {
        println!("{prompt}");
        let mut value = String::new();
        match reader.read_line(&mut value).await {
            Ok(_) => (),
            Err(_) => continue
        }

        let value = value.trim();

        if value.is_empty() && let Some(default) = default {
            return default;
        }

        if format_checker.valid(value) {
            return value.to_string();
        } else {
            println!("invalid input");
        }
    }
}

pub async fn read_int(prompt: &str) -> i32 {
    let mut reader = BufReader::new(io::stdin());
    loop {
        println!("{prompt}");
        let mut value = String::new();
        match reader.read_line(&mut value).await {
            Ok(_) => (),
            Err(_) => continue
        }

        let value = value.trim();

        match value.parse() {
            Ok(value) => {
                return value
            },
            Err(_) => {
                println!("invalid input");
                continue;
            }
        };
    }
}