#![allow(unused_variables)]
use utf8_slice;

// 难以管理

use std::mem;
use std::ops::Add;

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]

fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    //let mut f1 = File::from("f1.txt");
    //open(&mut f1);
    ////read(&mut f1,&mut vec![])
    //close(&mut f1);
    test_string();
    test_tuple();
    test_struct();
    test_enum();
}

fn test_string() {
    let s = String::from("hello world");
    let len = s.len();
    println!("len = {}", len);
    println!("s = {}", s);
    let slice = &s[0..2];
    println!("slice = {}", slice);
    let my_name = "Pascal";
    greet(my_name);

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    let slice = &s[..2];
    println!("slice = {}", slice);

    greet(&s[..3]);

    let s = "中国人";
    let a = &s[0..3]; //中文占用三个字节
    println!("a = {}", a);

    let mut s = String::from("hello world");
    let word = first_word(&mut s);
    println!("word = {}", word);
    s.clear();
    // println!("word = {}", word);

    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[1..3];
    println!("slice = {:?}", slice);

    let s = String::from("sssfd world");
    let slice = &s[0..1];

    let s = "hello:".to_string();
    let s1 = String::from(slice);
    println!("s = {}", s);
    println!("s1 = {}", s1);

    let mut s = String::from("hello world");
    s.push('中');
    s.push_str("国人");
    println!("s = {}", s);

    //测试String方法
    let mut s = String::from("hello world");
    let word = &s[0..5];
    let r = word.replace("h", "H");
    println!("r = {}", r);
    let rs = s.replace("he", "MM");
    println!("rs = {}", rs);
    let rs = s.replacen("l", "K", 1);
    s.replace_range(1..3, "KK");
    println!("rs = {}", s);

    let mut s = String::from("hello world");
    s.pop();
    println!("s = {:?}", s);
    s.remove(4);
    println!("s = {:?}", s);

    s.truncate(1); //裁剪函数
    println!("s = {:?}", s);
    s.clear();
    println!("s = {:?}", s);

    //测试连接
    let mut s = String::from("hello");
    let s1 = String::from("world");
    // let string_rust = String::from("rust");
    let slice = &s1[0..3];
    // let result = s + &string_rust;
    // let result = s + slice;
    // println!("result = {}", result);
    s += "rust";
    let s = s.add("rust");
    println!("s = {}", s);

    trans();

    for c in "中国人".chars() {
        println!("c = {}", c);
    }

    for b in "中国人".bytes() {
        println!("b = {}", b);
    }

    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // 如果你希望在字符串中使用双引号，可以使用以下形式
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果希望在字符串中使用 # 号，可以如下使用：
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 填空
    let long_delimiter = r###"Hello, \"##\""###;
    println!("{}", long_delimiter);

    for c in "中国人".chars() {
        println!("c = {}", c);
    }

    let s = "The 🚀 goes to the 🌑!";
    let rocket = utf8_slice::slice(s, 4, 5);
    println!("rocket = {}", rocket);

    //切片
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("slice = {:?}", slice);

    //字符串切片
    let s = String::from("hello world");
    let slice1 = &s[0..2];
    let slice2 = &s[0..=1];
    println!("slice1 = {}", slice1);
    println!("slice2 = {}", slice2);

    let s = "你好，世界";
    let slice = &s[0..3];
    let string: &mut String = &mut String::from("你好，世界");
    println!("string = {}", string);
    println!("slice = {}", slice);

    //字符串切片
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();

    let s = String::from("hello, 世界");
    let slice1 = &s[0..1];
    let slice2 = &s[7..10];
    for (i, c) in s.chars().enumerate() {
        println!("i = {}, c = {}", i, c);
    }

    let mut s = String::new();
    // let mut ss = String::with_capacity(32);
    s.push_str("hello");
    let v = vec![104, 101, 108, 108, 111];
    let s1 = String::from_utf8(v).unwrap();
    println!("s1 = {}", s1);

    //mem
    let story = String::from("Rust By Practice");
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();
    println!("ptr = {:p}", ptr);
    println!("len = {}", len);
    println!("capacity = {}", capacity);

    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
    println!("s = {}", s);

    // println!("word = {}", word);
}

fn test_tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("tup.0:{}", tup.0);
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn test_struct() {
    let user1 = User {
        email: String::from("someone@163.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("dfjasd@dff.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    let user2 = User {
        email: String::from("34odfu@dlfa.com"),
        ..user1
    };
    build_user(String::from("fasd@fjdal"), String::from("fasd"));
    // println!("user1 = {:?}", user1); //报错
    println!("user1 = {:?}", user1.active);
    println!("user2 = {:?}", user2);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(rect2);
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

//枚举

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8,
}

// 直接用枚举
enum PokerCard2 {
    Clubs(u8), //枚举值中带数据
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}
fn test_enum() {
    let options = Option::Some(5);
    println!("{:?}", options);

    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    print_suit(heart);

    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds, //diamond,
        value: 2,
    };

    let c1 = PokerCard2::Clubs(2);
    let c2 = PokerCard2::Diamonds('A');
    match_suit(c1);
    match_suit(c2);
}

fn print_suit(card: PokerSuit) {
    println!("card = {:?}", card);
}

fn match_suit(card: PokerCard2) {
    match card {
        PokerCard2::Clubs(value) => println!("Clubs {}", value),
        PokerCard2::Diamonds(value) => println!("Diamonds {}", value),
        PokerCard2::Spades(value) => println!("Spades {}", value),
        PokerCard2::Hearts(value) => println!("Hearts {}", value),
    }
}

//一般参数会使用切片，而不是String 通用性
fn greet(name: &str) {
    println!("hello {}", name);
}

fn first_word(s: &str) -> &str {
    &s[0..1]
}

fn trans() {
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    let long_string = "String literals\
                        can span multiple lines.\
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
