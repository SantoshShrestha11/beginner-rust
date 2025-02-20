use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time::Duration};

fn main() {
    let pb = ProgressBar::new(10_0000);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );

    for i in 0..10_0000 {
        do_hard_work();
        if i % 10_0000 == 0 {
            pb.tick(); // Updates without excessive prints
        }
        pb.inc(1);
    }

    pb.finish_with_message("✔ Done!");
}

fn do_hard_work() {
    thread::sleep(Duration::from_micros(10)); // Simulated work
}
