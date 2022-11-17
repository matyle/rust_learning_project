fn main() {
    let n = 6;
    if n % 4 == 0 {
        println!("{} is divisible by 4", n);
    } else if n % 3 == 0 {
        println!("{} is divisible by 3", n);
    } else if n % 2 == 0 {
        println!("{} is divisible by 2", n);
    }

    let condition = true;
    let n = if condition { 5 } else { 6 };
    println!("The value of n is: {}", n);

    //左闭右开
    for i in 1..5 {
        println!("i = {}", i);
    }

    let mut n = 0;
    while n < 5 {
        n += 1;
    }
    println!("n = {}", n);
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //loop
    let mut count = 0;
    let res = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        };
    };
    println!("res = {:?}", res);
}
