use std::process::id;

enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    // name: String,
}
fn main() {
    let dir = Direction::South;
    match dir {
        Direction::East => println!("East"),
        Direction::West | Direction::South => println!("South or North"),
        _ => println!("West"),
    }

    if let Direction::East = dir {
        println!("East");
    } else if let Direction::West = dir {
        println!("West");
    } else if let Direction::North = dir {
        println!("North");
    } else if let Direction::South = dir {
        println!("South");
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), //会匹配
        _ => println!("other"),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    stack();

    // let x = 5;
    let x = 4;
    match x {
        1..=5 => println!("1..=5"),
        _ => println!("other"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("other"),
    }

    // let p = Point {
    //     x: 0,
    //     y: 3,
    //     name: String::from("center"),
    // };
    // let Point { x, y, name } = p; //解构，拷贝i32，如果有String，会移动
    // println!("x = {}, y = {},name={:?}", x, y, name);

    // match p {
    //     Point { x, y: 0, name } => println!("x = {}, y = 0,name={:?}", x, name),
    //     Point { x: 0, y, name } => println!("x = 0, y = {}", y),
    //     // Point { x, y, name } => println!("x = {}, y = {}", x, y),
    //     _ => println!("other"),
    // }

    let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // let nums = [1, 3, 4, 5, 5, 6];
    // let nums = [3, 4, 5, 5, 6];
    let nums = [3, 4, 5, 5];
    match nums {
        [1, ..] => println!("1.."),
        [.., 6] => println!("..6"),
        [i, .., j] => println!("i={},j={}", i, j),
        // _ => println!("other"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("first={},last={:?}", first, last), //忽略中间值
    }

    //匹配守卫
    let num = Some(7);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("equal or greater: {}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n), //会匹配
        _ => println!("other"),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    //@

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
    let p @ Point { x, y } = Point { x: 0, y: 3 };
    println!("x = {}, y = {},p={:#?}", x, y, p); //x y为拷贝
    match 1 {
        num @ (1 | 2) => println!("num={}", num),
        _ => println!("other"),
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn stack() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
