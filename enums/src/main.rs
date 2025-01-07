// enum IpAddrKind{
//     V4,
//     V6,
// }
// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     
//     route(IpAdrrKind::v4);
//     route(IpAdrrKind::v6);
//
// }
// fn route(ip_kind: IpAddrKind) {}
// fn main() {
//     enum IpAddrKind{
//         V4,
//         V6,
//     }
//     //creating a struct to handle the data type of IpAddrKind.
//     //the type ip_kind was created using the enume.
//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }
//
//     let home = IpAddr {
//         kind:IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//     //here the address ::1 is similar to the home ip but in
//     //hexa decimal form 
// let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }
// that is how you can write the data directly inside into the enums
// with out using the structs
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

}
