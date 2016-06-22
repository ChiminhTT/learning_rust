fn main() {

    let x: Message = Message::Move { x: 3, y: 4 };

    enum BoardGameTurn {
        Move {
            squares: i32,
        },
        Pass,
    }

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

    // Construction as functions
    let m = Message::Write("Hello world".to_string());
    // is the same as
    fn foo(x: String) -> Message {
        Message::Write(x)
    }
    let x = foo("Hello World".to_string());
    // other use case
    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();

    for (index, message) in v1.into_iter().enumerate() {
        match message {
            Message::Write(s) => println!("Message {} is {}", index, s),
            _ => continue,
        }
    }
}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
}
