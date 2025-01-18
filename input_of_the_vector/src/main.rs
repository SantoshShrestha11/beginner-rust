use std::io;
fn main() {
    println!("Enter some thing here");
    let mut something  = String::new();
    io::stdin()
        .read_line(&mut something)
        .expect("failed to take the input form you ");
    println!("the thing that you have inputed is {}", something);
    // let some_vec: Vec<&str> = something.split_whitespace().s.parse<&i32>.collect();
    let some_vec: Vec<&int> =something;
        .something.split_whitespace()
        .filter_map(|&s| s.parse::<i32>().ok());
    println!("the data inside the some-vec is {:?}",some_vec);

    for words in some_vec {
        println!("{words}");
    }
}
