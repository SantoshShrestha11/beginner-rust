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

// #[cfg(target_os = "windows")]
// fn windows_only() {
//     println!("This only runs on Windows");
// }
//
// #[cfg(target_os = "linux")]
// fn linux_only() {
//     println!("This only runs on Linux");
// }
// fn main() {
//     linux_only();
//     windows_only();
// }
// fn main() {
//     if cfg!(target_os = "windows") {
//         println!("Running on Windows!");
//     } else {
//         println!("Not running on Windows!");
//     }
//     if cfg!(target_os = "linux") {
//         println!("Running on linux!");
//     } else {
//         println!("Not running on linux!");
//     }
// }

// fn drink(beverage: &str) {
//     // You shouldn't drink too much sugary beverages.
//     if beverage == "lemonade" {
//         //here are two types of the panics one is abort and another is unwind
//         //    unwind (default): The stack is unwound, destructors (drop) are called, and resources are cleaned up before terminating the thread.
//         // abort: The program immediately terminates without cleaning up resources, producing a smaller binary.
//         if cfg!(panic = "abort") {
//             println!("This is not your party. Run!!!!");
//         } else {
//             println!("Spit it out!!!!");
//         }
//     } else {
//         println!("Some refreshing {} is all I need.", beverage);
//     }
// }
//
// fn main() {
//     drink("water");
//     drink("lemonade");
// }
// #[cfg(panic = "unwind")]
// fn ah() {
//     println!("Spit it out!!!!");
// }
////here the type of the panic that we get is of unwind so another block of 
////code in not unwind is not taken in account
////also the panic strategy can be set form the command line using abort or unwind 
//// like this rustc projectname.rs -C panic=abort
// #[cfg(not(panic = "unwind"))]
// fn ah() {
//     println!("This is not your party. Run!!!!");
// }
//
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         ah();
//     } else {
//         println!("Some refreshing {} is all I need.", beverage);
//     }
// }
//
// fn main() {
//     drink("water");
//     drink("lemonade");
// }
// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
// fn give_adult(drink: Option<&str>) {
//     // Specify a course of action for each case.
//     match drink {//here inner is just a name of the variable so dont get 
//         //confused by the look of it
//         Some("lemonade") => println!("Yuck! Too sugary."),
//         Some(inner)   => println!("{}? How nice.", inner),
//         None          => println!("No drink? Oh well."),
//     }
// }
//
// // Others will `panic` before drinking sugary drinks.
// // All drinks are handled implicitly using `unwrap`.
// fn drink(drink: Option<&str>) {
//     // `unwrap` returns a `panic` when it receives a `None`.
//     let inside = drink.unwrap();
//     if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }
//
//     println!("I love {}s!!!!!", inside);
// }
//
// fn main() {
//     let water  = Some("water");
//     let lemonade = Some("lemonade");
//     let void  = None;
//
//     give_adult(water);
//     give_adult(lemonade);
//     give_adult(void);
//
//     let coffee = Some("coffee");
//     let nothing = None;
//
//     drink(coffee);
//     drink(nothing);
// }
// fn next_birthday(current_age: Option<u8>) -> Option<String> {
//     // If `current_age` is `None`, this returns `None`.
//     // If `current_age` is `Some`, the inner `u8` value + 1
//     // gets assigned to `next_age`
//     let next_age: u8 = current_age? + 1;
//     //here teh current_age? ? will just unwrap the value inside teh Option an the val here will 
//     //be u8
//     Some(format!("Next year I will be {}", next_age))
// }
