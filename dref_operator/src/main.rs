// use std::ops::dref;
struct MyBox<T>(T);
// here it holds one generic type T
impl<T>  MyBox<T>(T){
    fn new(x: T) -> MyBoc<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


fn main() {
    // here the value gets copied to the memory location of the b because of the type 
    let a: i32 = 43;
    let b: MyBox<i32> = MyBox::new(a);

    println!("Here value of the a is: {}",a);

    println!("Here value of the b is the memory location of the a: {:p}", b);

    println!("Here value of the b is the value of the a which can be gained by useing the drefrence operator: {}", *b);

    println!("using something else like this will work fine for printing the actual value there{}",&b);

    assert_eq!(43, *(b.deref()));

}
