use std::array::from_fn;
fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("arr has {} elements", arr.len());

    for v in arr {
        println!("{}", v);
    }

    //3重复7个
    let a = [3; 7];
    println!("a:{:?}", a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a:{:?}", a);
    //访问数组
    println!("a[0]:{}", a[0]);
    //越界访问
    // println!("a[5]:{}", a[5]);

    //切片
    let slice = &a[1..3];
    println!("slice:{:?}", slice);

    // let array = [String::from("hello"); 8]; //报错，不能copy
    // let array = [String::from("hello"), String::from("world")];
    // println!("array:{:?}", array);

    // let array: [String; 8] = core::array::from_fn(|i| String::from("hello"));
    // println!("array:{:#?}", array);
}
