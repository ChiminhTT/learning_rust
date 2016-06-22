use std::process;

fn main() {
    // Basic Match
    let x = 6;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Something else"),
    };

    // Matching on enums
    let msg_0 = Message::Write(String::from("This is a string"));
    let msg_1 = Message::Write(String::from("Another test"));
    let msg_2 = Message::Move { x: 32, y: 42 };
    let msg_3 = Message::ChangeColor(0, 0, 0);
    let msg_4 = Message::Quit;


    let messages = vec![msg_0, msg_1, msg_2, msg_3, msg_4];

    for msg in messages {
        process_message(msg);
    }

}

// Matching on enums
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
}

fn quit() {
    process::exit(3);
}
fn change_color(r: i32, g: i32, b: i32) {
    // ...
}
fn move_cursor(x: i32, y: i32) {
    // ...
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x: x, y: y } => move_cursor(x, y),
        Message::Write(s) => println!("{}", s),
    }
}
