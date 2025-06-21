#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    Resize { width: u32, height: u32 }, // varian struct
    Move { x: i32, y: i32 },             // varian struct
    Echo(String),                        // varian tuple
    ChangeColor(u8, u8, u8),             // varian tuple
    Quit,                                // varian unit (tanpa tanda kurung)
}


impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize { width: 10, height: 30 },
        Message::Move { x: 10, y: 15 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
