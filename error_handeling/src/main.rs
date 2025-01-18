use std::io;
fn main() {
    println!("Enter some numbers");
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("you got some error while taking the string");
    let nums:Result<i32, _> = string.split_whitespace().parse();
    match nums{
        Ok(number) => println!("the number taken as integer is {number}"),
        Err(error) => println!("you got some error while parsing {error}"),
    }

}
