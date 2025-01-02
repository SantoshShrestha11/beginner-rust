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

    let user2: User = build_user(String::from("sanish@gmail.com"), String::from("sanish"));
    let user3 = User{
        email: String::from("anish@gmail.com"),
        username: String::from("ansih"),
        ..user2
    };
}

fn build_user(email: String, username: String)->User{
    User{
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}
fn tuple_structs() {
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);
}
