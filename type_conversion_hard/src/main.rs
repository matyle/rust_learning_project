use std::convert;
use std::convert::TryInto;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    //类型转换 as
    let a: i32 = 10;
    let b: u16 = 100;
    if a < b as i32 {
        println!("a < b");
    } else {
        println!("a >= b");
    }

    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;

    println!("{},{},{}", a, b, c);

    let mut value: [i32; 2] = [1, 2];
    println!("{}, {}", value[0], value[1]);
    let p1: *mut i32 = value.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4; //i32占4个字节
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    println!("{}, {}", value[0], value[1]);

    //TyeInto转换

    let a: u8 = 10;
    let b: u16 = 1500;
    // let b_: u8 = b.try_into().unwrap(); //result 如果有错直接会panic
    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    if a < b_ {
        println!("ten is less than one hundred");
    }

    //&i32 实现了特征 Trait， &mut i32 可以转换为 &i32，但是 &mut i32 依然无法作为 Trait 来使用。
    let t: &i32 = &0;
    //使用特征对象
    foo(t);
    // foo(Box::new(t));
    let t1: &i32 = &10;
    // foo(Box::new(t1))
    //点操作符
    // let array :Rc<Box<[T;3]>> = ...;
    // let first = array[0]; //array.index(0)
}

fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}
//为什么没报错？因为值类型没有实现clone，就会推断引用，即&&value,其实就是地址，地址默认实现
fn do_stuffV2<T>(value: &T) {
    let cloned = value.clone();
}

#[derive(Clone)]
struct Continer<T>(Arc<T>);

fn clone_containers<T>(foo: &Continer<i32>, bar: &Continer<T>) {
    let foo_cloned = foo.clone();
    let bar_cloned = bar.clone();//为什么是&Continer<T>??
}

trait Trait {}
fn foo<X: Trait>(t: X) {}
impl<'a> Trait for &'a i32 {}

struct Foo {
    x: u32,
    y: u16,
}

struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}
