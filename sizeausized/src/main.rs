use std::mem::size_of;

fn main() {
    // let s1: str = "hello"; //报错：编译时期无法知道大小，同一类型的所有值应该是相同大小，所以只能使用引用了,类似指针大小固定大小，含有长度等信息
    let s1: &str = "hello";
    let size = size_of::<i32>();
    println!("size: {:?}", size);
    let size = size_of::<&str>(); //16 pointer , len
    println!("size: {:?}", size);
    let size = size_of::<usize>(); //8
    println!("size: {:?}", size);
    let size = size_of::<String>(); //24 pointer,len,cap
    println!("size: {:?}", size);
    let size = size_of::<Box<String>>(); //8
    println!("size: {:?}", size);

    // let s1: Box<str> = Box::new("hello:" as str);
    let s1: Box<str> = "hello".into();
    println!("{:?}", s1);
}

// fn my_unsize(n: usize) {
//     let array = [123; n]; //报错 在一个常量数组中使用非常量
// }
