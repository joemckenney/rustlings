// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
struct Coordinates {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Color(u32, u32, u32);

#[derive(Debug)]
enum Message {
    Move(Coordinates),
    Echo(String),
    ChangeColor(Color),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move(Coordinates { x: 10, y: 30 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(Color(200, 255, 255)),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
