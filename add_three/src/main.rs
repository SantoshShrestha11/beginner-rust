use std::io::{self, Write};

fn main() {
    match get_three_numbers() {
        Ok((num1, num2, num3)) => println!("sum: {}", sum(num1, num2, num3)),
        Err(err) => println!("Error: {}", err),
    }
}

fn get_number(prompt: &str) -> Result<i32, String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to real line")?;

    input
        .trim()
        .parse()
        .map_err(|_| "Please enter a valid integer!".to_string())
}

fn get_three_numbers() -> Result<(i32, i32, i32), String> {
    let num1 = get_number("Enter the first number: ")?;
    let num2 = get_number("Enter the second number: ")?;
    let num3 = get_number("Enter the third number: ")?;

    Ok((num1, num2, num3))
}

fn sum(a: i32, b: i32, c: i32) -> i32 {
    let sum: i32;
    sum = a + b + c;
    return sum;
}
