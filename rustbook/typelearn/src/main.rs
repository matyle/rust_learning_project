use std::str::{FromStr, Utf8Error};

use crate::traittype::{fly_static, fly_dyn};
use crate::traittype::{Fly, Pig};
mod traittype;

fn main() {
    //slice
    let arr = &mut [1, 2, 3];
    println!("{:?}", arr);
    arr[2] = 10;
    println!("{:?}", arr);
    let arr = vec![1, 2, 3, 4];
    println!("{:?}", arr);

    //str
    let truth: &'static str = "rustisgoodlanguage";
    println!("{:?}", truth.len());
    let ptr = truth.as_ptr();
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, truth.len() + 100);
        std::str::from_utf8(slice)
    };
    fn errorf<'a>(x: Utf8Error) -> &'a str {
        println!("error:{:?}", x);
        "err"
    }

    println!("{:?}", s.unwrap_or_else(errorf));
    //tuple
    let t: (i8, &str) = (1, "sd");
    println!("{:?}", t);
    //struct
    let p = People {
        name: String::from("Jim"),
        age: 13,
    };
    let c: char = '你';
    println!("{:?}", c.len_utf8());
    //enum
    let ipv4addr = IpAddr::V4(114, 114, 109, 108);
    let ipv6addr = IpAddr::V6(String::from("::1"));
    println!("{:?}", ipv4addr);
    println!("{:?}", ipv6addr);

    // smart pointer
    let box_people = Box::new(People {
        name: String::from("Tony"),
        age: 16,
    });
    println!("{:?}", box_people.name);

    // generic
    let p = Point { x: 1, y: 1 };
    let p1 = Point { x: 1.3, y: 1.4 };
    let p3 = DPoint { x: 1.4, y: 1.4 };
    let p4 = DPoint { x: 1, y: 1.4 };
    //trait

    let pig = Pig {
        action: String::from("sleep"),
    };
    assert_eq!(pig.fly(), false);

    assert_eq!(fly_dyn(&pig),false);
    assert_eq!(fly_static(pig), false);
    
}

//struct
struct People {
    name: String,
    age: i8,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

//泛型
enum Option<T> {
    Some(T),
    None,
}

struct Point<T> {
    x: T,
    y: T,
}
struct DPoint<T, U> {
    x: T,
    y: U,
}
