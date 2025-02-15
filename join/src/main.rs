use std::thread;
use std::time::Duration;

fn main() {
    let handel = thread::spawn(||{
        for i in 1..=10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    //The two threads continue alternating,
    //but the main thread waits because of the call to handle.join() 
    //and does not end until the spawned thread is finished.
    handel.join().unwrap()
}
