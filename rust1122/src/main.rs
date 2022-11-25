fn main() {
    // let mut s = String::from("hello world");
    // let s1 = &s;
    // let s2 = &s;
    // 不能同时有多个可变引用，而且可变引用不能和不可变引用“同时”存在
    // let s3 = &mut s;
    // println!("{}", s3);

    let s4;
    {
        let mut s = String::from("hello world");
        s4 = &s;
    } //s drop
      // println!("{}", s4);
      // println!("{}", s3);
      // 引用：为了不获得s的所有权，并且能够使用s
      // println!("{}", s);
      // let s = "hello world";
      // let s1;
      // {
      //     s1 = play_strv2(s);
      // }
      // println!("{}", s1);
    let y = String::from("xyz");
    let res; //y
    let x = String::from("helloworld rust");
    {
        // res = longest(x, y.as_str());
        // println!("{:?}", res);
        res = longestv2(&x, &y);
        println!("{:p}", x.as_ptr());
        // println!("{:p}", y.as_ptr());
    } //drop
    println!("{:?}", res);
    println!("{:p}", res.as_ptr());
}

// TODO:理解为什么会出现作用域结束，但是引用仍然可以使用的现象？
// 会不会有安全问题？
fn longestv2<'a, 'b>(x: &'a String, y: &'b String) -> &'a String
where
    'b: 'a,
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//生命周期：c cpp 里面垂悬指针 避免 垂悬引用

// fn play_str() -> &str {
//     let s = "hello"; //stack 上面
//     s
// }

//返回值的生命周期是和函数体无关，只和输入参数的生命周期有关
//fn play_strv2(x:  &str) -> &str {
//    //a代表
//    let s = "hello"; //stack 上面
//    s
//}
// 'a 一般习惯是a
// 返回值的生命周期：应该是取x，y里面生命周期较小的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
