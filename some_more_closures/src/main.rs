fn call_once<F>(func: F)
    where
    F: FnOnce(),
{
    func();
   }

fn consume_string<F>(f: F)
    where 
F: FnOnce(String),
{
 let message = String::from("hello, Rust");
    f(message);
}
fn main() {
    let name = String::from("Rust");

    let my_closure = move || {
        println!("Hello, {}!", name);
    };

    call_once(my_closure);
//call_once(my_closure); error my closure has been moved 


    let closure = move |text: String| {
        println!("{}",text);
    };
    consume_string(closure);
    //here in this case each time when i call the function a new ocpy of hte 
    //closure is passed in 
    consume_string(closure);

}
