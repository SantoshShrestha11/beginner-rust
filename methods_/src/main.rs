// #[derive(Debug)]
// struct Rectangle {
//     width: u32, 
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// impl Rectangle {// we can give the name of the method same as one of the struct's fields.
//     // we can only return the vals in the fields and do nothing else .
//     // methods like this are called getteres
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }
// fn main() {
//     let rect1= Rectangle {
//         width: 30,
//         height: 20,
//
//     };
//     if rect1.width () {
//         println!("the rectangle has a non zero width; it is {}", rect1.width);
//     }
//     println!("The area of the rectangle is {}", rect1.area());
// }
struct Rectangle{
    width: u32,
    height: u32,
}
// we can also create the multiple impl block for each struct, with it's own methods.
// here the impl block for the square - rectangle is created below in line 57. 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

 // this is how you can build new.
    // it is not a method and is is called for creating sq below in the main function.
//
impl Rectangle {
    fn square(size: u32) -> Self {
        Self{
        width: size,
        height: size,
        }
    }
}

fn main () {
    let sq = Rectangle::square(20);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 50,
    };
    let rect4 = Rectangle {
        width: 20,
        height: 10,
    };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("can rect1 hold rect4? {}", rect1.can_hold(&rect4));
    println!("can rect1 hold sq? {}", rect1.can_hold(&sq));

}
