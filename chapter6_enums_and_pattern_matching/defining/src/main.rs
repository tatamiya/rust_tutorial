enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option1<T> {
    Some(T),
    None,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let sume_number = Option1::Some(5);
    let some_string = Option1::Some("a string");

    let absent_nuber: Option1<i32> = Option1::None;

    let x: i8 = 5;
    let y: Option1<i8> = Option1::Some(5);
    // not allowed!: let sum = x + y;
}
