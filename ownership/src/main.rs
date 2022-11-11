fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    test_string();

    let s1 = String::from("hello");
    take_ownership(s1);

    // println!("s1 = {}", s1);
    make_copy(x);
    println!("x = {}", x);

    let s2 = String::from("hello_s2");
    let s2 = takes_and_gives_back(s2);

    test_reference(&s2);

    let mut s3 = String::from("hello_s3");
    let r1 = &mut s3; // 可变引用
    let r2 = &s3; // 不可变引用可以有多个
    let r3 = &s3;

    practice()
}

fn test_string() {
    let mut s = String::from("hello");
    s.push_str(", rust");
    println!("{}", s);

    // let s2 = s;
    let s2 = s.clone(); // deep copy 性能开销大
    println!("{}", s2);
    println!("{}", s);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn make_copy(i: i32) {
    println!("{}", i);
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn test_reference(s: &String) -> usize {
    s.len()
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
//
fn practice() {
    let s = String::from("hello");
    let mut s1 = s;
    s1.push_str(", world practice");
    println!("{}", s1);

    //
    let c = '中';
    let r1 = &c;
    let ref r2 = c;
    println!("r1 = {}, r2 = {}", r1, r2);
    println!("{:?},{:?}", get_addr(r1), get_addr(r2));
}

fn give() -> String {
    let s = String::from("hello");
    let _s = s.as_bytes();
    s
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
