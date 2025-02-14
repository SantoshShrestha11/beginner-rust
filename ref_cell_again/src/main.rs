use std::cell::RefCell;

fn basic_example() {
    let number = RefCell::new(53);

    println!("Original value: {}", *number.borrow());

    *number.borrow_mut() = 100;
    *number.borrow_mut() = 900;
    println!("New value: {}", *number.borrow());
}
#[allow(dead_code)]
fn some_testing() {
    let mut number: i32 = 5;

    println!("The Original number: {}", &number );

    number = 10;
    println!("The Original number: {}", &number );
    let some: &i32= &number;
    println!("The new number is: {}", &some);

}

fn main() {
    basic_example();
}

#[derive(Debug)]
struct Counter{
    value: RefCell<i32>,
} 

impl Counter {
    fn new() -> Counter {
        Counter {
            value: RefCell::new(0),
        }
    }
}
fn counter_example() {
    let counter = Counter{
        value: RefCell::new(0),
    };

    *counter.value.borrow_mut() += 1;
    println!("Counter: {:?}", counter);
}

fn multiple_refs() {
    let shared_data = RefCell::new(vec![1,2,3]);

    let reference = &shared_data;

    reference.borrow_mut().push(4);
    println!("Vector: {:?}", shared_data)
}


fn 
