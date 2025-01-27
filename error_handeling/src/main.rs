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

fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        //here are two types of the panics one is abort and another is unwind
        //    unwind (default): The stack is unwound, destructors (drop) are called, and resources are cleaned up before terminating the thread.
        // abort: The program immediately terminates without cleaning up resources, producing a smaller binary.
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!!");
        } else {
            println!("Spit it out!!!!");
        }
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
}
