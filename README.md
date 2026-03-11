# struct-input

A Rust crate for easy CLI input mapping to structs.

`struct-input` provides a `StructInput` trait that can be derived for any struct. This trait allows you to easily read CLI input and map it to the fields of your struct.

## Features

- Easy to use derive macro for the `StructInput` trait.
- Supports reading strings, integers, and optional strings.
- Customizable input prompts.

## Usage

Add `struct-input` and `struct-input-derive` to your `Cargo.toml`:

```toml
[dependencies]
struct-input = "0.1.0"
struct-input-derive = "0.1.0"
```

Then, derive the `StructInput` trait for your struct:

```rust
use struct_input::StructInput;

#[derive(StructInput)]
struct MyStruct {
    #[struct_input(prompt = "Enter your name: ")]
    name: String,
    #[struct_input(prompt = "Enter your age: ")]
    age: i32,
    #[struct_input(prompt = "Enter your favorite color (optional): ")]
    favorite_color: Option<String>,
}

fn main() {
    let my_struct = MyStruct::from_cli();
    println!("Hello, {}! You are {} years old.", my_struct.name, my_struct.age);
    if let Some(color) = my_struct.favorite_color {
        println!("Your favorite color is {}.", color);
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.