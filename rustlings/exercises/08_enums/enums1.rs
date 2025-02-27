#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(u32, u32),
    Move(i32, i32),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
