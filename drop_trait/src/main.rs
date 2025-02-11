struct CustomSmartPointer {
    data: String,
}

// The reason you can't change the name of the method is that this is part of the contract defined by
// the `Drop` trait. The `Drop` trait specifies a single method, `drop`, which is intended to be used
// for performing any necessary cleanup operations when an object is destroyed. By requiring that this
// method be named `drop`, Rust ensures that there's a consistent and predictable way to handle
// resource management across all types that implement the `Drop` trait.
//
// If you were to change the name of the method, it would break the contract defined by the `Drop`
// trait, causing Rust's automatic memory management to malfunction or fail entirely. This is why
// changing the name of the `drop` method is not allowed and goes against the language design
// principles that make Rust's memory safety guarantees possible.
//
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some stuff"),
    };
    // c.drop():
    // we cant sue that in place of that we can use the function that is provided by the 
    // rust standard library called drop if we want to clean up the memory
    // we dont have to bring it to the scope manually
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
}
