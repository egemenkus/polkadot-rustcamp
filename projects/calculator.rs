extern crate polkadot_rustcamp;
use polkadot_rustcamp::calculator;

fn main() {
    println!("Enter the first number:");
    let mut first_number = String::new();
    std::io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read number");
    let first_number: f64 = f64::from_str(first_number.trim()).expect("Invalid number");

    println!("Enter the operation (+,-,*,/)");
    let mut operation = String::new();
    std::io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read operation");
    let operation = operation.trim();

    println!("Enter the second number");
    let mut second_number = String::new();
    std::io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read number");
    let second_number: f64 = f64::from_str(second_number.trim()).expect("Invalid number");

    let operation_enum = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    let result = calculate(operation_enum);
    println!("Result: {}", result);
}
