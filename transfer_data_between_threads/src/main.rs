use std::sync::mpsc;
use std::thread;

fn main() {
    // you can always go to the defination gd
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg: String = String::from("hello there");
        // here send is taking the ownership
        tx.send(msg).unwrap();
        // here the send method sends the result type
    });

    let received: String = rx.recv().unwrap();
    // there is another method try_recv
    println!("God: {}", received);
}
