use std::cmp::Ordering;
fn main() {
    let a: i32 = 5;
    let b: i32 = 6;

    bigger(a ,b);


}
fn bigger(a: i32, b: i32) {
    match a.cmp(&b) {//here in this case the a is compared to the b 
        Ordering::Less => println!("b = {b} is greater then a = {a}"),
        Ordering::Greater => println!("a = {a} is greater then b = {b}"),
        Ordering::Equal => println!("a = {a} is equal to b = {b}"),
    }
}
