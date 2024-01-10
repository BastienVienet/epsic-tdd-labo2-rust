use std::io::{self, Write};

fn get_number(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        match num.trim().parse::<i64>() {
            Ok(n) if n >= i64::from(i32::MIN) && n <= i64::from(i32::MAX) => return n as i32,
            Ok(_) => println!("Number out of range, please try again"),
            Err(_) => println!("Invalid number, please try again"),
        }
    }
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtract(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn main() {
    loop {
        let mut input = String::new();
        print!("Enter operation (+, -) or 'q' to quit: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" {
            break;
        } else if input != "+" && input != "-" {
            println!("Invalid operation");
            continue;
        }

        let num1 = get_number("Enter first number: ");
        let num2 = get_number("Enter second number: ");

        match input {
            "+" => println!("Result: {}", add(num1, num2)),
            "-" => println!("Result: {}", subtract(num1, num2)),
            _ => unreachable!(),
        }
    }
}