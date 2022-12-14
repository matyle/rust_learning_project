// use std::f32;

// fn main() {
//     let pi = f32::consts::PI;
//     let radius = 5.00f32;

//     let area = pi * f32::powi(radius, 2);

//     println!(
//         "The area of a circle with radius {:.2} is {:.5}!",
//         radius, area
//     )
// }
// fn main() {
//     let mut res = 42;
//     let option = Some(12);
//     // res = match option {
//     //     Some(x) => x + res,
//     //     None => res,
//     // };
//     if let Some(x) = option {
//         res += x;
//     };
//     println!("{}", res);
// }
//
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // my_option.unwrap();
        println!("my_option is none");
    }

    let my_arr = &[-1, -2, -3 - 4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // my_empty_vec.resize(0, 5);
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    // let temp = value_a;
    // value_a = value_b;
    // value_b = temp;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
