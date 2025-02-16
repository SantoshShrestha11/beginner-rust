// use std::sync::Mutex;
// use std::thread;
//
// fn main() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];
//
//     for _ in 0..100 {
//         let handle = thread::spawn(move ||{
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {m:?}");
// }
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
    let counter = Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
} 
