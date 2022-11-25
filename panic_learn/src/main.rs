use std::net::IpAddr;
fn main() {
    let home: IpAddr = "258.0.0.1".parse().unwrap(); //会panic 需要错误处理
    println!("{}", home);
}
