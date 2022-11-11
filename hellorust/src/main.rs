fn main() {
    let penguin_data = "\
    common name,length(cm)
    Little penguin,33
    yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    // for record in records {
    // let fields: Vec<&str> = record.split(',').collect();
    // println!("{}", record.trim()); //去掉首尾空格
    // }

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // let field: Vec<_> = record.trim().split(',').collect();
        let field: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            println!("Record {:?}-> {:?}", record, field);
        }

        let name = field[0];

        if let Ok(length) = field[1].parse::<f32>() {
            println!("{} is {}cm length", name, length);
        }
    }
}
