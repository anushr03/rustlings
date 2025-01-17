#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Move {x: i8, y: i8},
    Echo (String),
    ChangeColor (i16, i16, i16),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
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
