pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button, width: {}, height: {}, label: {}",
            self.width, self.height, self.label
        );
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a select box,width:{},height:{} label:{}",
            self.width,
            self.height,
            self.options.join(",")
        );
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T> where T: Draw {}
//impl<T>为提前声明，Screen<T>为完整的结构体类型
//但是这种写法限制了 Screen 实例的 Vec<T> 中的每个元素必须是 Button 类型或者全是 SelectBox 类型。
//如果只需要同质（相同类型）集合，更倾向于这种写法：使用泛型和 特征约束，因为实现更清晰，且性能更好
//(特征对象，需要在运行时从 vtable 动态查找需要调用的方法)。
// impl<T: Draw> Screen<T> {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

impl Draw for u8 {
    fn draw(&self) {
        println!("Drawing a u8 : {}", *self);
    }
}

impl Draw for f64 {
    fn draw(&self) {
        println!("Drawing a f64 : {}", *self);
    }
}

fn draw1(x: Box<dyn Draw>) {
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

fn main() {
    // let x = 1.1f64;
    // let y = 8u8;
    // draw1(Box::new(x));
    // draw1(Box::new(y));

    // draw2(&x);
    // draw2(&y);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("yes"), String::from("maybe")],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
