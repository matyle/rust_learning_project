use std::mem::size_of;
fn main() {
    let v = vec![1, 2];
    println!("{:p}", v.as_ptr());
}
