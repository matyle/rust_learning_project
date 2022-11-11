fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);


    // let y = x;
    // println!("The value of y is:{}",y);

    //只能是常量表达式
    // const THO : u32 = 100 * 40;
    //报错
    // const YY : u32 = y;
    // println!("The value of THO is:{}",THO);

    x = x+1;
    {
        //新的作用域，可以使用相同的变量名
        let x = x+1;
        println!("The value of x is:{}",x);
    }

    println!("The value of x is:{}",x);

    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("The value of spaces is:{}",spaces);
    //
    // let mut space = "   ";
    // space = space.len();
    //
    
    let c = 'z';
    let z: char='Z';


    //tup
    let tup:(i32,f64,u8) = (500,6.4,1);
    println!("The value of tup 1 is {}",tup.1);
    let (x,y,z) = tup;
    println!("The value of x is:{}",x);

    
}
