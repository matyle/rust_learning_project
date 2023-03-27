trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

// pub struct SocketRead<'a> {
//     socket: &'a Socket,
// }
fn main() {
    println!("Hello, world!");
}
