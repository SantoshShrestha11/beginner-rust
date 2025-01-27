// use std::io;
// fn main() {
//     println!("Enter some numbers");
//     let mut string = String::new();
//     io::stdin()
//         .read_line(&mut string)
//         .expect("you got some error while taking the string");
//     let nums: Result<Vec<i32>, _> = string
//         .split_whitespace()
//         .map(|s| s.parse::<i32>())
//         .collect();
//     match nums{
//         Ok(number) => println!("the number taken as integer is {:#?}",number),
//         Err(e) => println!("you got some error while parsing {e}"),
//     }
//
// }
// fn drink(beverage: &str) {
//     if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }
//     //the things inside the panic will be printed
//     println!("Some refreshing {} is all I need.", beverage);
// }
//
// fn main() {
//     drink("lemonade");
// }

