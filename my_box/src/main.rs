use std::ops::Deref;
struct MyBox<T>(T);
// here the x is stored in the stack 
impl<T> MyBox<T>{
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }

}
fn hello(name: &str) {
    println!("Hello, {}!",name);
}
fn some_function(input: &str) {
    println!("The contents are: {}",input);
}

fn main() {
    let x:i32 = 5;
    let y: Box<i32> = Box::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);
    // the print function are more optimized
    println!("some val {}",*y);
    println!("some val {}",&y);
    println!("some val {}",y);


    let a:i32 = 10;
    let b: MyBox<i32> = MyBox::new(a);
    assert_eq!(10,a);
    assert_eq!(10,*b);
    assert_eq!(10,*(b.deref()));

    let m: MyBox<String> = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    let n:MyBox<String>= MyBox::new(String::from("Rusty"));
    some_function(&n);
    // here the thing is it was done automatically by the clippry deref but it is always the good 
    // practice to make it explicit
    // but it cannot perform the same thing when going from imutable reference to mutable
    // some_function(&(*n));
    some_function(&(*n)[..]);


}
