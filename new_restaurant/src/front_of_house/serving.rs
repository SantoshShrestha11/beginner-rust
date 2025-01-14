use crate::serve_customer; // import the `serve_customer` function from the root module

pub fn take_order() {
    println!("taking order.");
}

fn _serve_order() {
    println!("serving order");
}

pub fn take_payment() {
    println!("payment received.");
}

pub fn notify_parent() {
    // call the imported function directly
    serve_customer();
}
