struct Words<'a> {
    part: &'a String,
}

fn main() {
    let arr = [0; 1000]; //栈上
    let arr1 = arr; //发生拷贝
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let arr = Box::new([0; 1000]);
    let arr1 = arr; //堆上面所有权转移
    println!("{:?}", arr1.len());
    // println!("{:?}", arr.len());//err

    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    println!("{:?}", sum);

    let s = gen_static_str();
    println!("{}", s);

    // let x = 5;
    // let y = &x;
    // assert_eq!(5, *y);
    let x = Box::new(1);
    let sum = *x + 1;

    let x = MyBox::new(1);
    let sum = *x + 1;
    println!("{:?}", sum);

    //Deref 可以说是 Rust 中最常见的隐式类型转换，而且它可以连续的实现如 Box<String> -> String -> &str 的隐式转换，只要链条上的类型实现了 Deref 特征。
    let s = MyBox::new(String::from("hello"));
    let s1: &str = &s; //多次隐式解引用
    let s2: String = s.to_string();
}

//你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久，那么就可以使用 Box::leak
fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello");
    Box::leak(s.into_boxed_str())
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person {
            name: name,
            age: age,
        }
    }
    fn display(self: &mut Person, age: u8) {
        let Person { name, age } = &self; //为什么&&mut Person可以和Person匹配 因为deref解引用
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
