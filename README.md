# rust-conv
Convert integers, floating-point numbers, strings, and bools to each other using generics

# Usage:

- Step 1: Add log dependencies to your project as follows:

Cargo.toml:
````
[dependencies]
rust-log = { git = "https://github.com/gobkc/rust-conv" }
````

- Step 2: run `cargo update`
- Step 3: now you can use log the library

# Example

````
use rust_conv::conv;
fn main() {
    let integer_value: i32 = conv::any(42.5); // float to integer
    let float_value: f64 = conv::any(42); // integer to float
    let bool_value: bool = conv::any(1); // integer to bool
    let string_value: String = conv::any(true); // bool to string
    let bool_str_value: bool = conv::any("true".to_string()); // string to bool
    let str_int_value: u64 = conv::any("123".to_string()); // string to integer
    println!("bool_str_value: {} str_int_value: {}", bool_str_value, str_int_value);

    println!("Integer value: {}", integer_value); // output: Integer value: 42
    println!("Float value: {}", float_value); // output: Float value: 42.0
    println!("Bool value: {}", bool_value); // output: Bool value: true
    println!("String value: {}", string_value); // output: String value: true
}````
