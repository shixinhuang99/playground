fn main() {
    let a = 3;
    a.is_even().log();

    let b: i64 = 2443786374;
    b.is_even().log();

    let s = "hello";
    s.log();
}

trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

trait Log {
    fn log(&self);
}

impl Log for bool {
    fn log(&self) {
        println!("{}", self);
    }
}

impl Log for str {
    fn log(&self) {
        println!("{}", self);
    }
}
