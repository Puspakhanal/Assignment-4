use clap::{Arg, Command};
use std::process;
mod calculator;

fn main() {
    let matches = Command::new("cli-calc")
        .version("1.0")
        .about("A CLI calculator")
        .arg(
            Arg::new("operator")
                .short('o')
                .long("operator")
                .value_name("OPERATOR")
                .help("Sets the arithmetic operator")
                .value_parser(["+", "-", "*", "/", "add", "sub", "mul", "div"])
                .required(true),
        )
        .arg(
            Arg::new("operand1")
                .short('1')
                .long("operand1")
                .value_name("OPERAND1")
                .help("Sets the first operand")
                .required(true),
        )
        .arg(
            Arg::new("operand2")
                .short('2')
                .long("operand2")
                .value_name("OPERAND2")
                .help("Sets the second operand")
                .required(true),
        )
        .get_matches();

    let operator = matches.get_one::<String>("operator").unwrap();
    let operand1: f64 = matches
        .get_one::<String>("operand1")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("Error: Operand 1 must be a number");
            process::exit(1);
        });
    let operand2: f64 = matches
        .get_one::<String>("operand2")
        .unwrap()
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("Error: Operand 2 must be a number");
            process::exit(1);
        });

    // Debugging prints
    println!("Operator: {}", operator);
    println!("Operand1: {}", operand1);
    println!("Operand2: {}", operand2);

    match calculator::calculate(operator, operand1, operand2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}