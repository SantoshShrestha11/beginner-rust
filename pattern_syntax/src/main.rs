//here are the pattern syntax in rust 

#![allow(unused)]
fn main() {
    {let x: i32 = 1;

        match x {
            1 => println!("one"),
            2 | 9 | 10 => println!("two"),
            3..5 => println!("three"),
            20..=30 => println!("three"),
            _ => println!("anything"),
        }

    }

    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            //here y defined inside the Some will shadow the y in the let outside the this match
            //block
            Some(y) => println!("Matched, y = {:?}", y),
            //but here since the x is defined so the x will not be shadowed
            _ => println!("Default case, x = {:?}", x),
        }
    }

    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point{ x: 0, y: 7};

        let Point {x, y} = p;
        assert_eq!(0 , x);
        assert_eq!(7, y);
    }

    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point {x: 0, y: 7};

        match p {
            Point{x, y: 0} => {
                println!("On the x axis at {}", x)
            },
            Point{x: 0, y} => {
                println!("On the y axis at {}", y)
            },
            Point { x, y } => {
                println!("On neither axis: ({}, {})", x, y)
            },

        }
    }

    {
        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("Quit");
            }
            Message::Move { x, y } => {
                println!("Move to x: {}, y: {}", x, y);
            }
            Message::Write(text) => {
                println!("Text message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!(
                    "change color: red {}, green{}, and blue{}", r, g, g
                );
            }
        }
    }



    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(
            Color::Hsv(0, 160, 255)
        );

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!(
                    "Change color: red {}, green {}, and blue {}", r, g, b
                );
            }
            Message::ChangeColor(Color::Hsv(h , s , v))=> 
            {
                println!("Change color: hue {}, saturation {}, and value {}", h, s, v);
            }
            _ =>{
                println!("currently ignoring all")
            } 
        };
    }



{
        struct Point {
            x: i32,
            y: i32,
        }

        let ((feet, inches), Point{ x , y}) = 
        ((3,10), Point {x: 4, y: -10});

    }
    





{
        foo(3, 4);
        //here x is completely ignored 
        //due to the use of the use of _ in the function signature 
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
        }
    }





{
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
             _=> {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);
    }




{
        let numbers = (2,3,4,52,542);

        match numbers {
            (first, _, third, _ , fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth);
            }
        }
    }



{
        //here binds the val but does not gives the error too
        let _x =5;
        let y = 10;

        let s = Some(String::from("Hello!"));

        if let Some(_s) = s {
            println!("found a string");
        }

        //here _s takes the ownership so again trying to take the ownership in the print macro
        //here gives us the error 
        // println!("{:?}", s);

    }




    // here these are called the range syntax
    {
        struct  Point {
            x: i32, 
            y: i32, 
            z: i32,
        }


        let origin = Point {
            x: 0, y: 0, z: 0
        };


        match origin {
            Point {x, ..} => println!("x is {}", x),
        }
    }



{
        let numbers = (3,4,2,12,341,31,41,2,131,21,41,31);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
            
        }
    }




    //match gurads
{
let num = Some(4);
        match num {
            //here if x< 5 is called the match guard
            Some(x) if x< 5 => println!("less then five: {}", x),
            Some(x) => println!("{}",x),
            None => (),
        }
    }



{
        let x = Some(5);
        let y = 10;

        match x { 
            //here is another use of match guard
            Some(n) if n== y => println!("Matched, n = {}",n),
            _=> println!("Default case, x = {:?}", x),
        }
    }



{
        let x = 4;
        let y = false;

        match x {
            //here y has to be true in this match arm 
            3| 5| 6 if y => println!("yes"),

            _=> println!("no"),
        }
    }



    {
        enum Message {
            Hello {id: i32},
        }

        let msg: Message = Message::Hello { id: 5 };

        match msg {
            //here we are also storing the vals in hte id_variable using the @ operator
            //here it can be same as the id like 
               // id: id @3..=7,
            //} => println!("Found an id in range: {}", id),
            Message::Hello { 
                id: id_variable @3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            }

        }

    }
}
