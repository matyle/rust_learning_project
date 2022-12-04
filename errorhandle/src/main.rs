use std::fs::File;
use std::io

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}
fn main() -> Result<(), AppError> {
    let s1 = "hello";
    let s2 = String::from("world!");
    let l1 = handle_str(s1);
    let l2 = handle_str(&s2[..]);
    // let l3 = handle_string(s2); //move
    // println!("{:?}", l3);
    // match l3 {
    //     Ok(len) => println!("{:?}", len),
    //     Err(err) => println!("{:?}", err),
    // };
    // let r = l3.or(l1);
    // println!("{:?}", r);
    // or and
    let lerr = Err("hello");

    let r = l2.or(lerr);
    println!("{:?}", r);

    let r = l2.and(lerr);
    println!("{:?}", r);

    //filter
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;
    let fn_is_even = |x: &i8| x % 2 == 0;
    assert_eq!(s1.filter(fn_is_even), n);
    assert_eq!(s2.filter(fn_is_even), s2);
    //map map_err()

    //?
    let _file = File::open("non_txt")?;
    Ok(())
}

fn handle_str(s: &str) -> Result<usize, &str> {
    if s.len() > 0 {
        Ok(s.len())
    } else {
        Err("empty string")
    }
}

fn handle_string<'a>(s: String) -> Result<usize, &'a str> {
    if s.len() > 0 {
        Ok(s.len())
    } else {
        Err("empty string")
    }
}
