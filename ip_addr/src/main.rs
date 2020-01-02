#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Calling on {:?}.", &self);
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Hello");

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let home6 = IpAddrKind::V6(String::from("::1"));

    println!("{:?}, {:?}", home, home6);

    let m = Message::Write(String::from("tacos"));
    m.call();
}
