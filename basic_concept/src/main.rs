// use num::complex::Complex;
use std::{
    any::type_name,
    ops::{Range, RangeInclusive},
};

struct Struct {
    e: i32,
}
fn main() {
    // base_type();
    // practice();
    // statements_and_expressions();
    // _ = func_test(1, 2);
    // forever();
}

// fn add(i: i32, j: i32) -> i32 {
//     // return i + j;
//     i + j
// }
//

fn base_type() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    // // 常量
    const MAX_POINT: u32 = 100_000;
    println!("MAX_POINT = {}", MAX_POINT);

    //变量隐藏 作用域内对象和作用域外的对象可以同名
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("in x = {}", x);
    }
    println!("out x = {}", x);

    //字符串
    let s = "hello world";
    let s = s.len();

    //可变
    // let mut ss = "hello world";
    // ss = ss.len();
    println!("s = {}", s);

    // let guess = "43".parse::<i32>().expect("Not a number!");
    // println!("guess = {}", guess);

    //数值类型 溢出测试
    // let a: i8 = 127;
    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf).expect("read error");

    // let b: i8 = buf.trim().parse::<i8>().expect("parse error");
    // println!("a:{},b:{}", a, b);
    // println!("a+b = {}", a + b);

    //浮点数
    println!("浮点数");
    //assertion failed: 0.1 + 0.2 == 0.3'
    // assert!(0.1 + 0.2 == 0.3);
    //
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc(f32)");
    println!("  0.1+0.2:{:x}", (abc.0 + abc.1).to_bits());
    println!("      0.3:{:x}", (abc.2).to_bits());

    println!("xyz(f64)");
    println!("  0.1+0.2:{:x}", (xyz.0 + xyz.1).to_bits());
    println!("      0.3:{:x}", (xyz.2).to_bits());

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);
    //
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("x is nan");
    }

    for j in 1..10 {
        println!("j = {}", j);
    }

    //复数类型
    let a = num::complex::Complex { re: 1.0, im: 2.0 };
    println!("a = {}", a);

    //字符类型 4字节
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    let g = '国';

    println!(
        "c = {},z = {},heart_eyed_cat = {},g = {}",
        c, z, heart_eyed_cat, g
    );

    println!("{}，字符大小{}", c, std::mem::size_of_val(&c));
    println!("{}，字符大小{}", g, std::mem::size_of_val(&g));

    //单元类型() 类似于struct{}用来占位
}

fn practice() {
    // let x: i32 = 5;
    // let mut y: u32 = 6;
    // y = x; //error
    let v: u16 = 38_u8 as u16;
    println!("v = {}", v);

    let x: u32 = 5;
    // assert_eq!("u32".to_string(), type_of(&x));

    let v1 = 251_u8 + 4;
    let v2 = i8::checked_add(126, 1).unwrap();
    println!("v1 = {},v2 = {}", v1, v2);

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    let x = 1_000.000_1;
    let y: f32 = 0.12;
    let z = 0.01_f32;

    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.00001);

    let mut sum = 0;
    //左闭右开
    for i in -3..2 {
        println!("i = {}", i);
        sum += i;
    }
    assert_eq!(sum, -5);

    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    assert!((9.6_f64 / 3.2 - 3.0).abs() <= 0.00001);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); //1
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); //7
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); //6
    println!("1 << 5 is {}", 1u32 << 5); //32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); //32
}

fn type_of<T>(_: &T) -> String {
    // println!("typeid = {:?}", s.type_id());
    format!("{}", type_name::<T>())
}

//Statements and expressions
fn statements_and_expressions() {
    let y = {
        let x = 3; //语句
        x + 1 //没有分号，是表达式，有分号是语句
    };
    // println!("The value of y is: {:?}", y);
    println!("The value of y is: {:?}", y);

    let y = {
        let mut x = 3; //语句
        x += 1; //没有分号，是表达式，有分号是语句
                // println!("x = {}", x);
    };
    println!("The value of y is: {:?}", y);
}

fn func_test(x: i32, y: i32) -> i32 {
    println!("func_test");
    x + y
}

fn forever() -> ! {
    loop {
        println!("forever");
    }
}

fn print() -> () {
    println!("print");
}

fn test_match() {
    // 填空
    let b = false;

    match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
