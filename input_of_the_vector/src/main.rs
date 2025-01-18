use std::io;
fn main() {
    println!("Enter some thing here");
    let mut something  = String::new();
    io::stdin()
        .read_line(&mut something)
        .expect("failed to take the input form you ");
    println!("the thing that you have inputed is {}", something);
    let some_vec: Vec<i32> = something
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("failed to parse integer"))
        .collect();
    println!("the data inside the some-vec is {:?}",some_vec);

    if some_vec.is_empty() {
        println!("no valid integers were provided here");
    }
}
