use std::ops::{Div, Mul};

use rust_decimal_macros::dec;

fn main() {
    let number = dec!(-1.23);
    let number1 = dec!(-2.0);
    assert_eq!("-1.23", number.to_string());
    println!("{}", number.mul(number1));
    println!("{}", number * number1);
    println!("{}", number.div(number1));
}
