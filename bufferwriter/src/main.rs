use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    writeln!(handle, "foo: {}", 42);
}
