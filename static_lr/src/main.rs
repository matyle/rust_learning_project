use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn get_mem_location() -> (usize, usize) {
    let string = "Hello world";
    let ptr = string.as_ptr() as usize;
    let length = string.len();
    (ptr, length)
}

fn get_str_from_ptr(ptr: usize, len: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(ptr as *const u8, len)) }
}

use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: &T) {
    println!("{:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("{:?}", input);
}

fn main() {
    let (ptr, len) = get_mem_location();
    let msg = get_str_from_ptr(ptr, len);
    println!("{:?},{:?}", ptr, len);
    println!("The {} bytes at 0x{:X} stored: {}", len, ptr, msg);

    // const i: i32 = 5;
    let i = 5;
    print_it(&i);
    // print_it1(&i);
    let r1;
    let r2;
    {
        static static_var: i32 = 42;
        r1 = &static_var;
        let x = "'a static str";
        r2 = x;
    }
    print_it(r1);
    print_it(&r2);
}
