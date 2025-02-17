#![allow(unused)]
fn main() {
    //irrefutable
    let x = 5;

    //refutable 
    let x:Option<&str> = None;
    if let Some(x) = x {
        println!("{}",x);
    };


    let a: Option<&str> = None;
    //here a is refutable but the Some needs the irrefutable so we get an error here 
    let Some(a) =  a;

    if let x = 5 {
        println!("{}",a);
    };
}
//can only accept irrefutable patterns :
//function parameters 
//let statements
//for loops
