fn example1() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn example2() {
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn example3() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn example4() {
    let _x = 5;

    let (_x, _y, _z) = (1, 2, 3);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn example5() {
    let point = (3, 5);
    print_coordinates(&point);

    let print_coordinates_closure = |point: &(i32, i32)| {
        let (x, y) = point;
        println!("Current location: ({}, {})", x, y);
    };

    print_coordinates_closure(&point);
}

fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
}
