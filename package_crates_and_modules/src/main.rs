use package_crates_and_modules::{add, multiply};

fn main() {
    let a = 5;
    let b = 3;

    let sum = add(a,b);
    let product = multiply(a,b);

    println!("Main program :");
    println!("The sum of {} and {} is {}",a,b, sum);
    println!("The product of {} and {} is {}",a,b, product);
}
