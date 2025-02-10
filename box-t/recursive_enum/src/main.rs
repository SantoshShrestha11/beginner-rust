#[allow(dead_code)]
enum List {
    //Cons have two fields first is i32 and second is List which is recursive
    Cons(i32, Box<List>),
    // on stack the box is a pointer to the heap so it has a known size
    Nil,
    // and here the nill does not heap any data so it has a known size
}

use List::{Cons, Nil};
#[allow(unused_variables)]
fn main() {
    // here type of the list is List and it is a recursive enum so we have to use Box to store it on heap
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
