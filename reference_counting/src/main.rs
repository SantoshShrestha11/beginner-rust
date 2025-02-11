use std::rc::Rc;
enum List{
    Cons(i32,Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::new(Cons(3, a.clone()));
    println!("count after creating b = {}", Rc::strong_count(&a));

    let c = Rc::new(Cons(4, Rc::clone(&a)));
    println!("count after creating c = {}", Rc::strong_count(&a));
}
