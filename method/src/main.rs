struct Circle {
    x: f64,
    y: f64,
    redius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, redius: f64) -> Circle {
        Circle { x, y, redius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.redius * self.redius)
    }
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn width(&self) -> f64 {
        self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move x: {}, y: {}", x, y),
            Message::Write(s) => println!("write: {}", s),
            Message::ChangeColor(r, g, b) => println!("change color r: {}, g: {}, b: {}", r, g, b),
        }
    }
}
fn main() {
    let c = Circle::new(0.0, 0.0, 2.0);
    println!("area of circle:{}", c.area());
    let rect1 = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("area:{:?}", rect1.area());
    println!("width:{:?}", rect1.width());

    let m = Message::Write(String::from("hello"));
    m.call()
}
