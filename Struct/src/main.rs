struct User{
    username : String,
    email: String,
    sign_in_count: u64,
    active: bool
}
fn main() {
    let mut user1=User{
        email: String::from("santosh@gmail.com"),
        username: String::from("Santosh Shrestha"),
        sign_in_count: 1,
        active: true
    };
    let name = user1.username;
    println!("The name of the user is {name}.");
    user1.username = String::from("Sanish Shrestha");
    println!("The name of user after changing is {0}.",user1.username);
    println!("The name of the uer after changing too but the refrence is different {0}.",name);
}
