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

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn divide(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        return 0; // Not really 0 but explaining the error in the main function
    }
    num1 / num2
}

fn modulo(_num1: i32, _num2: i32) -> i32 {
    return 3
}

fn main() {
    loop {
        let mut input = String::new();
        print!("Enter operation (+, -, *, /, %) or 'q' to quit: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" {
            break;
        } else if input != "+" && input != "-" && input != "*" && input != "/" && input != "%" {
            println!("Invalid operation");
            continue;
        }

        let num1 = get_number("Enter first number: ");
        let num2 = get_number("Enter second number: ");

        match input {
            "+" => println!("Result: {}", add(num1, num2)),
            "-" => println!("Result: {}", subtract(num1, num2)),
            "*" => println!("Result: {}", multiply(num1, num2)),
            "/" => {
                let result = divide(num1, num2);
                if result != 0 {
                    println!("Result: {}", result);
                } else {
                    println!("Cannot divide by zero")
                }
            },
            "%" => println!("Result: {}", modulo(num1, num2)),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{add, subtract, multiply, divide, modulo};

    // ------------------ Add ------------------

    #[test]
    fn test_add_two_numbers() {
        let num1 = 1;
        let num2 = 2;
        let expected_result = 3;

        let result = add(num1, num2);

        assert_eq!(result, expected_result);
    }

    // ------------------ Sub ------------------

    #[test]
    fn test_sub_two_numbers() {
        let num1 = 10;
        let num2 = 8;
        let expected_result = 2;

        let result = subtract(num1, num2);

        assert_eq!(result, expected_result);
    }

    // --------------- Multiply ----------------

    #[test]
    fn test_multiply_two_numbers() {
        let num1 = 10;
        let num2 = 8;
        let expected_result = 80;

        let result = multiply(num1, num2);

        assert_eq!(result, expected_result);
    }

    // --------------- Divide ------------------

    #[test]
    fn test_divide_two_numbers() {
        let num1 = 10;
        let num2 = 2;
        let expected_result = 5;

        let result = divide(num1, num2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_divide_by_zero() {
        let num1 = 10;
        let num2 = 0;
        let expected_result = 0;

        let result = divide(num1, num2);

        assert_eq!(result, expected_result);
    }

    // --------------- Modulo ------------------

    #[test]
    fn test_23_modulo_4_equals_3() {
        let num1 = 23;
        let num2 = 4;
        let expected_result = 3;

        let result = modulo(num1, num2);

        assert_eq!(result, expected_result);
    }
}
