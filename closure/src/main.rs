use std::{thread, time::Duration};

fn main() {
    let x = 1;
    let sum = |y, z| x + y * z;
    println!("{:?}", sum(2, 4));

    let intensity = 10;
    let random_number = 7;
    workout(intensity, random_number);
}

fn muuuuu(intensity: u32) -> u32 {
    println!("muuuuuu.....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// |param1, param2,...| {
//     语句1;
//     语句2;
//     返回表达式
// }
fn workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("{}个俯卧撑！", muuuuu(intensity))
    } else if random_number == 3 {
        println!("{}个俯卧撑！", muuuuu(intensity))
    } else {
        println!("{}个俯卧撑！", muuuuu(intensity))
    }
}

fn workoutv2(intensity: u32, random_number: u32) {
    let action = || {
        println!("muuuuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    if intensity < 25 {
        println!("{}个俯卧撑！", action())
    } else if random_number == 3 {
        println!("{}个俯卧撑！", action())
    } else {
        println!("{}个俯卧撑！", action())
    }
}
