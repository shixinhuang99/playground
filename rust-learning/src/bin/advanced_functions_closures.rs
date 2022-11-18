fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
}

fn returns_closures() -> Box<dyn Fn(i32) -> i32> {
    const FIVE: i32 = 5;
    Box::new(|x| x + FIVE)
}

fn returns_fn() -> Box<fn(i32) -> i32> {
    fn add_two(x: i32) -> i32 {
        x + 2
    }

    Box::new(add_two)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    println!("list_of_strings: {:?}", list_of_strings);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("list_of_statuses: {:?}", list_of_statuses);

    let a = 1;
    let add_five_closure = returns_closures();

    println!("a + 5 = {}", add_five_closure(a));

    let add_one_fn = returns_fn();

    println!("a + 2 = {}", add_one_fn(a));
    // println!("a + 2 = {}", (*add_one_fn)(a));
}
