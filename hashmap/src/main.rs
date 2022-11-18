use std::collections::HashMap;
fn main() {
    let mut gems: HashMap<&str, i32> = HashMap::new();
    // gems.insert("红宝石", 1);

    let mut gems_cap = HashMap::with_capacity(128);
    gems_cap.insert("luck", "但愿人长久");

    let team_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), -2),
    ];

    let teams_map: HashMap<_, _> = team_list.into_iter().collect();
    //遍历map 是无须的，顺序随机
    //如果需要有序，可以先把map中的元素取出来，放到vec中，排序
    println!("{:?}", teams_map);

    let name = String::from("sunface"); //未实现copy trait
    let age = 18; //基本类型实现了copy trait

    let mut boy = HashMap::new();
    // boy.insert(name, age);
    boy.insert(&name, age);

    // std::mem::drop(name);
    // let n = name;
    // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    // println!("{}", age);
    for b in boy {
        println!("{:?}", b);
    }

    // let score: Option<&i32> = teams_map.get("日本队");
    let score: Option<&i32> = teams_map.get("小日本队");
    match score {
        Some(s) => println!("{:?}", s),
        None => println!("key is nil"), // println!("{:?}", score);
    }
}
