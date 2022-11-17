#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}
#[derive(Debug)]
enum Gender {
    Male(User),
    Female(User),
}

fn main() {
    let u1 = Gender::Male(User {
        name: "u1".to_string(),
        age: 18,
    });

    let u2 = Gender::Male(User {
        name: "u2".to_string(),
        age: 19,
    });

    let u3 = Gender::Male(User {
        name: "u3".to_string(),
        age: 19,
    });

    let u4 = Gender::Female(User {
        name: "u4".into(),
        age: 18,
    });

    let users = vec![u1, u2, u3, u4];

    // 找出 所有男性
    // let males = matches!(users,if )
    let males = users
        .iter()
        .filter(|user| matches!(user, Gender::Male(_)))
        .collect::<Vec<_>>();
    println!("males: {:?}", males);

    // 找出 19 岁的男性
    // let male_19 = users.iter().filter(|user| matches!(user, Gender::Male(_)));
    // println!("male_19: {:?}", male_19);
    let males_19 = users
        .iter()
        .filter(|user| matches!(user, Gender::Male(male) if  male.age== 19))
        .collect::<Vec<_>>();
    println!("males: {:?}", males_19);

    // 找出 19 岁男性中不叫 u2 的人
    let male_19_not_u2 = users
        .iter()
        .filter(|user| matches!(user, Gender::Male(male) if  male.age==19&&male.name!="u2"))
        .collect::<Vec<_>>();
    println!("male_19_not_u2: {:?}", male_19_not_u2);

    // 找出 18 岁的所有人
    let age_18s = users
        .iter()
        .filter(|user| matches!(user, Gender::Male(male) if male.age==18)
        .collect::<Vec<_>>();

    println!("age_18s: {:?}", age_18s);
}
