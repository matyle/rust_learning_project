// fn main() {
//     let ori = Box::new(5);
//     println!("ori: {}", ori);
//     let stolen = ori;
//     println!("stolen: {}", stolen);
//     // println!("ori: {}", ori); // error: use of moved value: `ori`
//     let mut i = 20;
// }
//
// fn computer(input: &i32, output: &mut i32) {
//     let input_cache = *input;
//     if input_cache > 10 {
//         *output = 1;
//     } else if input_cache > 5 {
//         *output *= 2;
//     }
// }
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest<'a: 'b, 'b>(x: &'a str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
