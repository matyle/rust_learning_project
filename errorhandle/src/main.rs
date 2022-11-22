fn main() {
    let s1 = "hello";
    let s2 = String::from("world!");
    let l1 = handle_str(s1);
    let l2 = handle_str(&s2[..]);
    let l3 = handle_string(s2);
    // println!("{:?}", l3);
    match l3 {
        Ok(len) => println!("{:?}", len),
        Err(err) => println!("{:?}", err),
    };
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
