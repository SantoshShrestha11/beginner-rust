use std::io;
fn main() {
    println!("Enter some integers leaving one space");
    let mut input  = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to take the input form you ");
    let int: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("failed to parse"))
        .collect();
    println!("the sum of the integers is {}",sum(int));
}
fn sum(integers: Vec<i32>) -> i32 {
    integers.iter().sum()
}
