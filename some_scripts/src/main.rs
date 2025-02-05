use std::process::Command;

fn main() {
    let output = Command::new("ls") // Replace with "dir" on Windows
        .arg("-la")  // Pass arguments like in the terminal
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

