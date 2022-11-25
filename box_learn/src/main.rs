struct Words<'a> {
    part: &'a String,
}

fn main() {
    let arr = [0; 1000]; //栈上
    let arr1 = arr; //发生拷贝
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let arr = Box::new([0; 1000]);
    let arr1 = arr; //堆上面所有权转移
    println!("{:?}", arr1.len());
    // println!("{:?}", arr.len());//err
}
