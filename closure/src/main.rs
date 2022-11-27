use std::{thread, time::Duration};

fn muuuuu(intensity: u32) -> u32 {
    println!("muuuuuu.....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// |param1, param2,...| {
//     语句1;
//     语句2;
//     返回表达式
// }
fn workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("{}个俯卧撑！", muuuuu(intensity))
    } else if random_number == 3 {
        println!("{}个俯卧撑！", muuuuu(intensity))
    } else {
        println!("{}个俯卧撑！", muuuuu(intensity))
    }
}

fn workoutv2(intensity: u32, random_number: u32) {
    let action = || {
        println!("muuuuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    if intensity < 25 {
        println!("{}个俯卧撑！", action())
    } else if random_number == 3 {
        println!("{}个俯卧撑！", action())
    } else {
        println!("{}个俯卧撑！", action())
    }
}

fn main() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    println!("{:?}", v2);

    let s = "hell0".to_string();
    //所有权移动
    let update_string = || println!("{}", s);
    let upstr = |str| println!("{}", str);
    // update_string("wold");
    exec(update_string);
    exec3(upstr);
    // println!("{}", s);
    // let x = 1;
    // let sum = |y, z| x + y * z;
    // println!("{:?}", sum(2, 4));

    // let avg = |y, z| y * z / 2;
    // println!("{:?}", avg(2, 4));

    // let intensity = 10;
    // let random_number = 7;
    // workout(intensity, random_number);

    let f = factory();
    let res = f(1);
    println!("{}", res);

    let f = factory();
    let res = f(5);
    println!("{}", res);
    fn fn_elision(x: &i32) -> &i32 {
        x
    }

    let closure_elision = |x: &i32| -> &i32 { x };
}
fn exec3<'a, F: Fn(&'a str)>(mut f: F) {
    f("hello");
}

//闭包作为返回值
// fn factory() -> Fn(i32) -> i32 {
//     let num = 5;
//     |x| x + num
// }

fn factory_box(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}
fn factory() -> impl Fn(i32) -> i32 {
    let num: i32 = 5;
    move |x| x + num
}

fn exec<F: FnOnce()>(f: F) {
    f();
}

fn exec1<F: FnMut()>(mut f: F) {
    f();
}

fn exec2<F: Fn()>(mut f: F) {
    f();
}

//结构体中的闭包
//TODO:思考是否有不使用Copy值特征实现的方法？

struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct CacherRef<'a, T, E: 'a>
where
    T: Fn(&'a E) -> &'a E,
{
    query: T,
    value: Option<&'a E>,
}

impl<'a, T, E: 'a> CacherRef<'a, T, E>
where
    T: Fn(&E) -> &E,
{
    fn new(query: T) -> Self {
        CacherRef { query, value: None }
    }

    fn value(&mut self, arg: &'a E) -> &'a E {
        match self.value {
            Some(v) => &v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn fn_onece<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn fn_onece<T>(func: T)
where
    T: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}
