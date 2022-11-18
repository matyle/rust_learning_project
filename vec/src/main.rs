fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 0..17 {
        v.push(i);
    }
    println!("{}", v.capacity());
    let cap = 1024;
    let mut v2: Vec<i32> = Vec::with_capacity(cap);
    println!("{}", v2.capacity());

    let v3 = vec![1, 5, 3, 4];
    println!("{}", v3.capacity());

    //Vector 与其元素共存亡
    //元素被引用怎么办？能不能死？肯定不能死

    let third = v3[1];
    println!("{}", third);

    //v.get()对越界做了特殊处理，不会有安全问题
    match v3.get(1) {
        Some(third) => println!("第三个元素是{}", third),
        None => println!("NO"),
    }

    //同时借用多个元素
    let mut v = vec![1, 2, 34, 5, 6];
    let frist = &v[0];
    //原因在于：数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存，这非常 rusty —— 对用户进行严格的教育。

    // v.push(7);
    println!("first:{}", frist);

    // let v = vec![
    //     IpAddr::V4("127.0.0.1".to_string()),
    //     IpAddr::V6("::1".to_string()),
    // ];
    // for ip in v.iter() {
    //     show_addr(ip);
    // }
    // for ip in &v {
    //     show_addr(ip);
    // }
    let v5: Vec<Box<dyn IpAddrT>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];
    for ip in v5 {
        ip.display()
    }
}

fn show_addr(ip: &IpAddr) {
    println!("{:?}", ip);
}

trait IpAddrT {
    fn display(&self);
}
struct V4(String);
impl IpAddrT for V4 {
    fn display(&self) {
        println!("ipv4:{:?}", self.0);
    }
}

struct V6(String);

impl IpAddrT for V6 {
    fn display(&self) {
        println!("ipv6:{:?}", self.0);
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
