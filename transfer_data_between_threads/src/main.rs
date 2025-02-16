use std::sync::mpsc;
use std::thread;
use std::time::Duration;
//
// fn main() {
//     // you can always go to the defination gd
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let msg: String = String::from("hello there");
//         // here send is taking the ownership
//         tx.send(msg).unwrap();
//         // here the send method sends the result type
//     });
//
//     let received: String = rx.recv().unwrap();
//     // there is another method try_recv
//     println!("God: {}", received);
// }
//
// fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//     for received in rx {
//         println!("Got: {}", received);
//     }
// }
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    //here we are creating a clone of the channel so that we can send the data
    //to the other thread

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
