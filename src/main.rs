use std::io::{self, Write, Read, BufRead, BufReader};

// The get_number function reads a line from the provided reader and tries to parse it as an i32.
// If the input can't be parsed as an i32, it prints an error message and tries again.
fn get_number<R: Read>(prompt: &str, reader: R) -> i32 {
    let mut reader = BufReader::new(reader);
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut num = String::new();
        reader.read_line(&mut num).unwrap();
        match num.trim().parse::<i32>() {
            Ok(n) => return n,
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

fn modulo(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        return 0; // Returning 0 because modulo by 0 is undefined
    }
    ((num1 % num2) + num2) % num2
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

        let num1 = get_number("Enter first number: ", io::stdin());
        let num2 = get_number("Enter second number: ", io::stdin());

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
            "%" => {
                let result = modulo(num1, num2);
                if num2 != 0 {
                    println!("Result: {}", result);
                } else {
                    println!("Cannot modulo by zero because it is undefined")
                }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{add, subtract, multiply, divide, modulo, get_number};

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

    #[test]
    fn test_10_modulo_5_equals_0() {
        let num1 = 10;
        let num2 = 5;
        let expected_result = 0;

        let result = modulo(num1, num2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_5_modulo_minus_3_equals_minus_1() {
        let num1 = 5;
        let num2 = -3;
        let expected_result = -1;

        let result = modulo(num1, num2);

        assert_eq!(result, expected_result);
    }

    // ----------- Get Number --------------

    #[test]
    fn test_get_number_invalid_input() {
        let input = Cursor::new(b"abc\n42\n");
        let result = get_number("Enter a number: ", input);

        assert_eq!(result, 42);
    }
}
