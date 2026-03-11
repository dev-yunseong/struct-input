# struct-input

A Rust crate for easy CLI input mapping to structs.

`struct-input` provides a `StructInput` trait that can be derived for any struct. This trait allows you to easily read CLI input and map it to the fields of your struct.

## Features

- Easy to use derive macro for the `StructInput` trait.
- Supports reading strings, integers, and optional strings.
- Customizable input prompts.

## Usage

Add `struct-input` to your `Cargo.toml`:

```toml
[dependencies]
struct-input = "0.1.2"
```

Then, derive the `StructInput` trait for your struct:

### Example
```rust
use struct_input::{StructInput, Format};

#[derive(StructInput, Debug)]
struct ServerConfig {
    // Rejects input if it contains spaces
    #[struct_input(
        message = "Service Name: ",
        format = "NotAllowWhitespace",
    )]
    service_name: String,

    // Validates URL format and provides a default value
    #[struct_input(
        message = "API Endpoint: ",
        format = "BaseUrl",
        default = "https://localhost:8080"
    )]
    endpoint: String,

    // Automatically parses string input to i32
    #[struct_input(
        message = "Max Retries: "
    )]
    retries: i32,
}

async fn main() {
    let config = ServerConfig::from_input().await;
    println!("{:#?}", config);
}
```

### Attribute Reference
| Attribute | Type |Description|
|-|-|-|
|message|String|The label displayed to the user when requesting input.|
|format|String|Validation rule: BaseUrl, Name, NotAllowWhitespace, or None.|
|default|String|The value used if the user provides empty input (presses Enter).|
|message|String|An additional hint or error message displayed to the user.|