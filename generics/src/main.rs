// fn add_i8(a: i8, b: i8) -> i8 {
//     a + b
// }
// fn add_i32(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn add_f64(a: f64, b: f64) -> f64 {
//     a + b
// }

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b //报错
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<i32> {} //为i32实现单独Point

fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    let arr: [i32; 3] = [1, 2, 3];
    display_arr_v3(arr);
    // display_arr(&arr);
    let arr: [i32; 2] = [1, 3];
    // display_arr(arr); //报错，类型不匹配
    // display_arr(&arr); //引用可以
    //
    display_arr_v3(arr);

    #[derive(Debug)]
    struct A;
    #[derive(Debug)]
    struct S(A); //元组结构体
    #[derive(Debug)]
    struct SGen<T>(T);

    let a = A;
    let s = S(a);
    println!("s: {:?}", s);

    let str = "hello时间";
    check_size(str);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn check_size<T>(_: T) {
    println!("{}", std::mem::size_of::<T>());
}

fn display_arr(arr: &[i32]) {
    for i in arr.iter() {
        println!("{}", i);
    }
}
fn display_arr_v2<T: std::fmt::Debug>(arr: &[T]) {
    for i in arr.iter() {
        println!("{:?}", i);
    }
}

fn display_arr_v3<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    for i in arr.iter() {
        println!("{:?}", i);
    }
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }
