#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {// we can give the name of the method same as one of the struct's fields.
    // we can only return the vals in the fields and do nothing else .
    // methods like this are called getteres
    fn width(&self) -> bool {
        self.width > 0
    }
}
fn main() {
    let rect1= Rectangle {
        width: 30,
        height: 20,

    };
    if rect1.width () {
        println!("the rectangle has a non zero width; it is {}", rect1.width);
    }
    println!("The area of the rectangle is {}", rect1.area());
}
