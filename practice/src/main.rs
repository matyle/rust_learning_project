#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}

struct Unit;
trait SomeTrait {
    //定义一些行为
}

impl SomeTrait for Unit {
    //实现行为
}

fn main() {
    // let age = 30;
    // let p = Person {
    //     name: String::from("sunface"),
    //     age,
    //     hobby: "sing,dancing,rap,play basketball".to_string(),
    // };
    let person = Person {
        name: String::from("Jack"),
        age: Box::new(30),
    };

    let Person { name, ref age } = person;
    println!("name: {}, age: {}", name, age);

    let u = Unit;
    do_something_with_unit(u);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    if let Some(n) = six {
        println!("six: {}", n);
    }

    //链表
    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.Len());
    println!("{}", list.stringify());

    //条件和循环
    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("index: {}, value: {}", i, v);
    }

    let boolean = true;
    let number = match boolean {
        true => 1,
        false => 0,
    };

    #[derive(PartialEq)] //使用==运算符
    enum MyEnum {
        Foo,
        Bar,
    }

    let o = Some(7);

    let mut v = String::from("hello");
    let r = &mut v;
    let mut s = r;
    // match r {
    //     mut value => {
    //         value.push_str("world");
    //     }
    // }
    // println!("{}", r);
    println!("{}", v);
}


fn do_something_with_unit(u: Unit) {}
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}
fn rec() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1); //会格式化打印
}

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn file() {
    let f = File {
        name: String::from("hello.txt"),
        data: String::from("hello,world"),
    };
    let _name = f.name; //所有权转移
                        // println!("f is {:#?}", f);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

use crate::List::*;

enum List {
    // Cons: 元组，将一个元素和一个指向下一个节点的指针组合在一起
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    //返回一个空的List
    fn new() -> List {
        Nil
    }

    //老链表 新增结点，返回新链表
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    //返回链表长度
    fn Len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.Len(),
            Nil => 0,
        }
    }
    fn stringify(&self) -> String {
        match self {
            Cons(headval, ref tail) => {
                format!("{},{}", headval, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}
