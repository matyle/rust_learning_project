fn main() {
    let num: [usize; 3] = [1, 2, 3];
    let r1 = &num as *const usize; //reference to pointer
                                   // let r2 = &mut num as *mut i32; // mut reference to mut pointer

    //## use exist ref to create pointer is safe behavior
    // deref is unsafe
    unsafe {
        // println!("rl is {}", *r1);
        let r = r1.read_volatile();
        println!("r is {}", r);
        // let num: usize = 10;
        // r1 = &num as *const usize;
        // println!("r is {}", r);
        // println!("rl is {}", *r1);

        let arr: [usize; 10] = [0; 10];
        let nums: &[usize] = core::slice::from_raw_parts(r1.add(1), 3);
        println!("{:?}", arr);
        println!("{:?}", nums);
    }

    let s: &str = "ecd";
    let ptr: *const u8 = s.as_ptr();
    unsafe {
        println!("{}", *ptr.add(0) as char);
        println!("{}", *ptr.add(2) as char);
    }

    fn nums_app() {}
    let nums_app_ptr = nums_app as *const usize;
    println!("{:?}", nums_app_ptr);

    // use address to create a pointer is unsafe
}
