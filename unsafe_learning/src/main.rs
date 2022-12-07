fn main() {
    let mut num = 5;
    let r1 = &num as *const i32; //reference to pointer
    let r2 = &mut num as *mut i32; // mut reference to mut pointer

    //## use exist ref to create pointer is safe behavior
    // deref is unsafe
    unsafe {
        println!("rl is {}", *r1);
    }

    // use address to create a pointer is unsafe
}
