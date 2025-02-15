use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(||{
        for i in 1..10 {
            println!("hi number {i} form teh spawned thread!");
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_secs(1))
        //here all the threads are running in the background and the time Duration is 1 sec so 
        //the main thread is sleeping for 1 sec and the spawned thread is sleeping for 1 ms
        //hence the output will be like 1 2 3 4 5 1 2 3 4 5
    }
}
