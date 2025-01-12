//#[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     Califonia,
//     NewYok,
//     Texas,
//     Florida,
//     Nevada,
//     Hawaii,
//     Washington,
//     Arizona,
// }
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// //in the match function the evaluation can be in any type
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         //here Coin::Penny is called pattern and the things after => is called expression.
//         //each things are called arms and they are seperated by using the commas
//         Coin::Penny =>{
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter forn {state:?}!");
//             25
//         },
//         //here the resultant value of the matching expression is the returning value.
//         //we can should use the curly braces for the expresson , if the expression is short then we
//         //can avoide it
//     }
// } 
//
// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }
//
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("the val stored here is {:#?}",six);
//     println!("the val stored here in the None thing is {:#?}",none);
// }
//
// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//     }
//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn move_player(num_spaces: u8) {}
//     }
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
       // _ => reroll(),
        _ => (), 
        // we have use the emptly tuple type
    }
    // the symbol _ here is catch all pattern 
    // we can use it to prevent compile time errors when a match isn't exhaustive
    // it can also sevre as a place holder when we dont care about the value beign matched
    // if catch-all pattern is not presented then all cases aren't coverend , in which case teh
    // rust will reaise the compile time rror because teh match expresson must be exhausetive
    // it is an good idea to not relay on it

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    //fn reroll() {}
}
