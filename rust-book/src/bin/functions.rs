fn main() {
    println!("main");
    print_labeled_measurement(5, 'h');
    println!("{}", foo());
}

fn print_labeled_measurement(value: i32, unit_label: char) -> () {
    println!("The measurement is: {value}{unit_label}");
}

fn foo() -> i32 {
    let val = {
        let x = 3;
        x + 1
    };

    val
}
