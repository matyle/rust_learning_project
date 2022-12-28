use std::{
    cell::{LazyCell, RefCell},
    sync::{Arc, LazyLock, Mutex},
};

//default shared between threads
// static A: RefCell<i32> = RefCell::new(3);
// static A: Arc<RefCell<i32>> = Arc::new(RefCell::new(3));
// static A: Arc<Mutex<i32>> = Arc::new(Mutex::new(3));
//Lazy::new

static B: LazyLock<Arc<Mutex<i32>>> = LazyLock::new(|| Arc::new(Mutex::new(3)));
fn main() {
    println!("Hello, world!");
}
