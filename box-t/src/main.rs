use std::any::type_name_of_val;

fn main() {
    let b = Box::new(6);
    println!("b = {}", &b);
    println!("The type of the b is {}", type_name_of_val(&b));
    println!("Reference: {:#?}", &b);

    // we use the formator {:p} to print the memory location of the variable
    println!("Mem location: {:p}", &b);
}
