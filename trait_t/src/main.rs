struct Container(i32, i32);

// 使用关联类型实现重新实现以下特征
// trait Contains {
//    type A;
//    type B;

trait Contains<A, B> {
    fn contains(&self, t1: &A, t2: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}
trait Containsv2 {
    type A;
    type B;
    fn contains(&self, t1: &Self::A, t2: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the last number.
    fn last(&self) -> i32 {
        self.1
    }
}
//
impl Containsv2 for Container {
    type A = i32;
    type B = i32;
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // Grab the first number.
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the last number.
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
    container.last() - container.first()
}
// fn difference<A, B, C>(container: &C) -> i32
// where
//     C: Contains<A, B>,
// {
//     1
// }
//
fn differencev2<C: Containsv2>(container: &C) -> i32 {
    // type A = i32;
    // type B = i32;
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {},v2:{}",
        &number_1,
        &number_2,
        Contains::contains(&container, &number_1, &number_2),
        Containsv2::contains(&container, &number_1, &number_2),
        // Container::contains(&container, &number_1, &number_2);
        // Containerv2::contains(&container, &number_1, &number_2)
    );
    // println!("First number: {}", container.first());
    // println!("Last number: {}", container.last());

    println!("The difference is: {}", differencev2(&container));
    println!("The difference is: {}", difference(&container));
}
