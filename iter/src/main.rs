use std::{
    collections::btree_map::Iter,
    ops::{Div, Range},
    ptr::eq,
};

fn main() {
    let s = "hello";
    let mut c = s.chars();
    let mut string = String::new();

    match c.next() {
        None => println!("None"),
        Some(first) => {
            string.push(first.to_ascii_uppercase());
            for cc in c {
                string.push(cc)
            }
        }
    }

    let numbers = vec![27, 297, 38502, 81];
    let v1_iter = numbers.iter();
    for val in v1_iter {
        println!("{}", val);
    }

    let arr = [1, 2, 3];
    // let mut arr_iter = arr.into_iter();
    // assert_eq!(arr_iter.next(), Some(1));
    // assert_eq!(arr_iter.next(), Some(2));
    // assert_eq!(arr_iter.next(), Some(3));
    // assert_eq!(arr_iter.next(), None);

    let result = match IntoIterator::into_iter(arr) {
        mut iter => loop {
            match iter.next() {
                Some(x) => println!("{}", x),
                None => break,
            }
        },
    };
    println!("{:?}", result)
}
