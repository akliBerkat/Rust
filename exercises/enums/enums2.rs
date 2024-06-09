// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// Same as the first one but here we notice the power of rust when it comes to tuples and the
// efficiency of it's types handeling 

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Echo(String),
    Move { x: u32, y: u32},
    ChangeColor(u32, u32, u32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
