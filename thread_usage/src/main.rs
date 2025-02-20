use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly ..");
    thread::sleep(Duration::from_secs(secs));
    println!("Done calculating");
    intensity
}
fn main() {
    simulated_expensive_calculation(14);
}
