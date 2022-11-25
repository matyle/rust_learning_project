#![allow(unused)]
fn main() {
    let r = 1;
    // {
    //     let x = 5;
    //     r = &x; //悬垂指针
    // }
    println!("r:{}", r);
    {
        // let r;                // ---------+-- 'a 这是r的生命周期
        //                       //          |
        // {                     //          |
        //     let x = 5;        // -+-- 'b  |
        //     r = &x;           //  |       |
        // }                     // -+       |
        //                       //          |
        // println!("r: {}", r); //          |
    } // ------------------------------------+
      //在编译期，Rust 会比较两个变量的生命周期，结果发现 r 明明拥有生命周期 'a，但是却引用了一个小得多的生命周期 'b，在这种情况下，编译器会认为我们的程序存在风险，因此拒绝运行。
    {
        let x = 5; //              x的生命周期更长
        let r = &x;
        println!("{}", r); //没问题
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    // let result = longest()

    //生命周期的语法也颇为与众不同，以 ' 开头，名称往往是一个单独的小写字母，
    //大多数人都用 'a 来作为生命周期的名称。
    //如果是引用类型的参数，那么生命周期会位于引用符号 & 之后，并用一个空格来将生命周期和引用参数分隔开:

    let x = 10;
    let num = &x;

    // println!("{:?}", longestV2());
    //
    // let i;
    // {
    //     let noval = String::from("helloworld. some years ago...");
    //     let first_sentence = noval.split('.').next().expect("Can not find '.'");
    //     i = ImportantExcerpt {
    //         part: first_sentence,
    //     };
    // }
    // println!("{:?}", i);
    let ss;
    {
        let s: &'static str = "hello";
        // let s = "helo";
        ss = s;
    }
    // println!("{:?}", ss);
}

//返回引用，需要标志生命周期，因为不知道返回是x还是y
//'a 为生命周期标记
fn longestV1(x: &str) -> &str {
    let a = "hello";
    a
}

//然后把字符串的所有权转移给调用者
// fn longestV2<'a>() -> &'a str {
//     let a = "hello";
//     a
// }
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
        // x.to_string()
    } else {
        y
        // y.to_string()
    }
}

fn useless<'a>(first: &'a i32, second: &'a i32) {}

//该生命周期标注说明，结构体 ImportantExcerpt
//所引用的字符串 str 必须比该结构体活得更久。
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//手动说明生命周期b一定比a小
//'a 必须比 'b 活得久
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    // impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'a str
// where
    //     'a: 'b,
    {
        self.part
    }
    fn announce_and_return_other(&'a self, announcement: &'b str) -> &'b str {
        self.part
    }
}
