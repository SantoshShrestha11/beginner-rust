use std::env;
fn main() {
    //here while running the following line of the code the first thing that is stored is
    //the path of the executable
    for argument in env::args() {
        println!("{argument}");
    }

{
        let a = [2,5,2,32,42,5];
        println!("{:?} , {:?}",a.iter().nth(4), a.iter().nth(4));
        println!("{:?}",a.iter().nth(4));
        println!("{:?}",a.iter().nth(4));
        // here nth returns teh sum type
    }
}
