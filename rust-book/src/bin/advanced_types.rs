type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn _forever() -> ! {
    loop {
        println!("forever")
    }
}

fn _generic<T: ?Sized>(_t: &T) {
    // 
}

fn main() {
    let a: i32 = 1;
    let b: Kilometers = 2;

    println!("a + b = {}", a + b);

    let _thunk: Thunk = Box::new(|| println!(""));

    let guess = String::from("1");

    loop {
        let _guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
}
