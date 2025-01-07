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
fn main() {
    enum IpAddrKind{
        V4,
        V6,
    }
    //creating a struct to handle the data type of IpAddrKind.
    //the type ip_kind was created using the enume.
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind:IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
