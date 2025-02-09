// #[allow(unused_imports)]
// use add_one;
// use rand;
// use add_two;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

    let another_num = 20;
    println!("Hello, world! {another_num} plus two is {}!", add_two::add_two(another_num));
}
