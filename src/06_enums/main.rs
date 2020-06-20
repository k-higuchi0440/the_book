fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    let m1 = Message::Write(String::from("hello"));
    m1.operate();

    let m2 = Message::Move { x: 10, y: 10 };
    m2.operate();

    let some_u8 = Some(0u8);
    if let Some(3) = some_u8 {
        println!("three!")
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: u32, y: u32 }, // anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn operate(&self) {
        match self {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move to x:{}, y:{}", x, y),
            Message::Write(m) => println!("message is {}", m),
            Message::ChangeColor(r, g, b) => println!("color is r:{}, g:{}, b:{}", r, g, b),
        }
    }
}
