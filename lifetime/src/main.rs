// struct Interface<'a> {
//     manager: &'a mut Manager<'a>,
// }

// impl<'a> Interface<'a> {
//     pub fn noop(self) {
//         println!("interface consumed");
//     }
// }

// struct Manager<'a> {
//     text: &'a str,
// }

// struct List<'a> {
//     manager: Manager<'a>,
// }

// impl<'a> List<'a> {
//     pub fn get_interface(&'a mut self) -> Interface {
//         Interface {
//             manager: &mut self.manager,
//         }
//     }
// }

// fn main() {
//     let mut list = List {
//         manager: Manager { text: "hello" },
//     };

//     list.get_interface().noop();

//     println!("Interface should be dropped here and the borrow released");

//     use_list(&list);
// }

// fn use_list(list: &List) {
//     println!("{}", list.manager.text);
// }
struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>,
}

impl<'b, 'a> Interface<'b, 'a>
where
    'a: 'b,
{
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
    where
        'a: 'b,
    {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
