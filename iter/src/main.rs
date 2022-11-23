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
    let division_results = numbers.into_iter().map(|n| divide(n, 27)); //会移动值
    println!("{:?}", division_results);
    // let division_results = numbers.iter().map(|n| divide(n, 27));
}

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    // if num <= 1 {
    //     return num;
    // }
    // num * factorial(num - 1)
    // Iter::from(num).fold(1, |acc, x| (x - 1) * x)
    (1..=num).fold(1, |acc, x| (x - 1) * x)
}
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    // todo!();
    //是否整除？

    if b == 0 {
        Err(String::from("divide by zero"))
    } else if a % b == 0 {
        Ok(a / b)
    } else {
        Err(String::from("not evenly"))
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|x| capitalize_first(x)).collect()
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|x| capitalize_first(x)).collect()
}

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut result = String::new();
    match c.next() {
        None => String::new(),
        Some(first) => {
            result.push(first.to_ascii_uppercase());
            for cc in c {
                result.push(cc)
            }
            result
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

use std::collections::HashMap;
fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    // todo!();
    map.values()
        .into_iter()
        .filter(|x| **x == value)
        .fold(0, |acc, _| acc + 1)
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]

    // collection.iter().for_each(|m| {
    //     m.values()
    //         .into_iter()
    //         .filter(|x| **x == value)
    //         .fold(0, |acc, _| acc + 1)
    // })
    collection
        .iter()
        .map(|x| {
            x.values()
                .into_iter()
                .filter(|v| **v == value)
                .fold(0, |acc, _| acc + 1)
        })
        .sum()
}
