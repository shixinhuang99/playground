#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let _v: Vec<i32> = Vec::new();

    let mut v = vec![1, 2, 3];
    v.push(4);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in &mut v {
        *i *= 2;
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
