struct Ipv4Addr {
    // --snip--
}
struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);
fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 1, y: 2 };
    let m3 = Message::Write(String::from("hello"));

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y.unwrap();
    let sum = x + y.unwrap_or(0);
    println!("sum={}:", sum);

    let y: Option<i32> = Some(23);
    let res = plus_one(y);
    println!("res={:?}:", res.unwrap());
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn HandleMessageV1(msg: Message) {
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move x: {}, y: {}", x, y),
        Message::Write(s) => println!("Write {}", s),
        Message::ChangeColor(r, g, b) => println!("ChangeColor r: {}, g: {}, b: {}", r, g, b),
    }
}
