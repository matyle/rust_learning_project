pub trait Summary {
    fn summarize_author(&self) -> String; //只需要实现这个方法
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// pub trait Display {
//     fn display(&self) -> String;
// }
pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    // fn summarize(&self) -> String {
    //     // println!("{} by {}", self.title, self.author);
    //     format!("{} by {}", self.title, self.author)
    // }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// impl Display for Post {
//     fn display(&self) -> String {
//         format!("Display:{} by {}", self.title, self.author)
//     }
// }

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

// impl Display for Weibo {
//     fn display(&self) -> String {
//         format!("DisPlay Weibo{}: {}", self.username, self.content)
//     }
// }

//语法糖
// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

//特征约束
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
//     println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
// }

//多重约束
// pub fn notify(item: &(impl Summary + Display)) {
//     println!("Breaking news! {} {}", item.summarize(), item.display());
// }
//
// pub fn notify<T: Summary + Display>(item: &T) {
//     println!("Breaking news! {} {}", item.summarize(), item.display());
// }

// fn some_function<T: Display + Clone, U: Clone>(t: &T, u: &U) -> i32 {
//     0
// }

fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("weibo"),
        content: String::from("weibo content"),
    }
}

// fn returns_summarizable_error(switch: bool) -> impl Summary {
//     if switch {
//         Weibo {
//             username: String::from("weibo"),
//             content: String::from("weibo content"),
//         }
//     } else {
//         Post {
//             title: String::from("title"),
//             author: String::from("author"),
//             content: String::from("content"),
//         }
//     }
// }

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone,
{
    0
}

// use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x={}", self.x);
        } else {
            println!("y={}", self.y);
        }
    }
}

use std::ops::Add;
#[derive(Debug)]
struct Point<T: std::ops::Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main() {
    let post = Post {
        title: String::from("Hello"),
        author: String::from("Jack"),
        content: String::from("Hello world"),
    };
    let weibo = Weibo {
        username: String::from("XiaoDingDang"),
        content: String::from("I am happy"),
    };
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    let weibo1 = Weibo {
        username: String::from("Mary"),
        content: String::from("I am sad"),
    };

    let p1 = Point {
        x: 1.1f32,
        y: 2.2f32,
    };
    let p2 = Point {
        x: 3.3f32,
        y: 4.4f32,
    };
    println!("{:?}", add(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p3, p4));
    // notify(&weibo);
    // notify(&post);
    // notify(&weibo);
    //
    let f = File::new("test.txt");
    println!("{:?}", f);
    println!("{}", f);
}
