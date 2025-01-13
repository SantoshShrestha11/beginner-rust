// fn main () {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {max}"),
//         _=> (),
//     }
// }
// we can write the following code in place of the above code using the if let 
// the above code is boilerplate code that it id repetitive , standard or necessary but dont
// add much functional value to the program
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max{
        println!("The maximum is configured to be {max}");
    }
}
//here it wont work it is the snippit of the another code block 
fn main() {
    let mut count = 0;
    // match coint {
    //     Coin::Quarter(state) => println!("State quarter form {state:?}!"),
    //     _ => count += 1,
    if let Count::Quarter(state) = coin {
        println!("State quarter form {state:?}!");
    }else {
        count += 1;
    }
   // }
}
//we can write this in the following way using the if let statement.
