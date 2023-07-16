extern crate polkadot_rustcamp;
use polkadot_rustcamp::concatenate_strings;

fn main() {
    let string1 = String::from("Hello, ");
    let string2 = String::from("Rust!");

    concatenate_strings::main(&string1, &string2);
}
