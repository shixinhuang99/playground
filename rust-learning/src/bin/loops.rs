fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);

    loop_label();
    example_while();
    example_for();
}

fn loop_label() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);
}

fn example_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("{}", number);
}

fn example_for() {
    let arr = [1, 2, 3];
    for element in arr {
        println!("{}", element);
    }

    for num in (1..3).rev() {
        println!("{}", num);
    }
}
