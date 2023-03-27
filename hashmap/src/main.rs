// use std::collections::HashMap;
// fn main() {
//     let mut gems: HashMap<&str, i32> = HashMap::new();
//     // gems.insert("红宝石", 1);
//
//     let mut gems_cap = HashMap::with_capacity(128);
//     gems_cap.insert("luck", "但愿人长久");
//
//     let team_list = vec![
//         ("中国队".to_string(), 100),
//         ("美国队".to_string(), 10),
//         ("日本队".to_string(), -2),
//     ];
//
//     let teams_map: HashMap<_, _> = team_list.into_iter().collect();
//     //遍历map 是无须的，顺序随机
//     //如果需要有序，可以先把map中的元素取出来，放到vec中，排序
//     println!("{:?}", teams_map);
//
//     let name = String::from("sunface"); //未实现copy trait
//     let age = 18; //基本类型实现了copy trait
//
//     let mut boy = HashMap::new();
//     // boy.insert(name, age);
//     boy.insert(&name, age);
//
//     // std::mem::drop(name);
//     // let n = name;
//     // println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
//     // println!("{}", age);
//     for b in boy {
//         println!("{:?}", b);
//     }
//
//     // let score: Option<&i32> = teams_map.get("日本队");
//     let score: Option<&i32> = teams_map.get("小日本队");
//     match score {
//         Some(s) => println!("{:?}", s),
//         None => println!("key is nil"), // println!("{:?}", score);
//     }
// }
#![allow(unused)]
use std::collections::HashMap;
use std::hash::Hash;
fn main() {
    // let mut map = HashMap::new();
    // let mut key = "test";
    let mut key = String::from("test");
    // map.insert(key, 12);
    // let value = get_default(&mut map, key);
    // // let value = get_default_ref(&mut map, key);
    // println!("value: {}", value);
    //
    let mut vec = vec![1, 2, 3];

    let mut_ref = &mut vec; // 创建一个可变引用
    let reborrow1 = &mut *mut_ref; // 创建一个新的可变引用
                                   // let reborrow2 = &mut *mut_ref; // 再次创建一个新的可变引用

    // 对可变引用进行操作
    // *mut_ref = vec![4, 5, 6];
    *reborrow1 = vec![7, 8, 9];
    // *reborrow2 = vec![10, 11, 12];

    println!("{:?}", vec); // 输出 [10, 11, 12]
}
fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> V
where
    K: Clone + Eq + Hash,
    V: Default + Copy,
{
    match map.get_mut(&key) {
        Some(value) => *value,
        None => {
            map.insert(key.clone(), V::default());
            let v = map.get_mut(&key).unwrap();
            *v
        }
    }
}
// //map至少要比value活的久
// fn get_default_ref<'a, 'm, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'a mut V
// where
//     'm: 'a,
//     K: Clone + Eq + Hash,
//     V: Default,
// {
//     let v = map.get_mut(&key);
//     if v.is_none() {
//         map.insert(key.clone(), V::default());
//         map.get_mut(&key).unwrap()
//     } else {
//         v.unwrap()
//     }
// }
