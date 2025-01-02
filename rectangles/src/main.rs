// fn main() {
// let    width1: u32 = 20;
//     let length1: u32 = 30;
//     println!("The area of the rectangle is {0}", area(width1, length1));
//
// }
//
// fn area(width: u32, length: u32) -> u32 {
//     width * length
// }

// fn main() {
//     let rect:(u32,u32) = (20,30);
//     println!("the area of the function is {0}", area(rect));
//
// }
//
// fn area(dimension:(u32,u32)) -> u32 {
//     dimension.0*dimension.1
// }
#[derive(Debug)]
struct Rect{
    length: u32,
    width:u32
}
fn main() {
    let rect1 = Rect {
        length: 20,
        width: 30
    };
//The {:?} with the # prefix in println! is a valuable tool for inspecting and presenting your data in a clear and readable format,
    //aiding in both debugging and understanding your program's behavior.
    println!("The area is {}",area(&rect1));
    println!("rectangle {:#?}",rect1);

}

fn area(rectangle:&Rect) -> u32 {
    rectangle.length * rectangle.width
}
