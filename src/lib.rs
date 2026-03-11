mod common;
mod util;

pub use util::formats::Format;
pub use util::{read_string, read_string_option, read_int};
pub use common::StructInputTrait;
pub use struct_input_derive::StructInput;
