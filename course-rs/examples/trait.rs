use std::{fmt, ops};

fn main() {
    let a = 3;
    a.is_even().log();

    let b: i64 = 2443786374;
    b.is_even().log();

    let s = "hello";
    s.log();

    is_even_and_log(&a);
    is_even_and_log_box(Box::new(b));

    let mut v = VecWrapper(vec![String::from("hello"), String::from("world")]);
    println!("{}", v);
    v.push(String::from("foo"));
    println!("{}", v);
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

trait Log: fmt::Display {
    fn log(&self) {
        println!("{}", self);
    }
}

impl Log for bool {}

impl Log for str {}

fn is_even_and_log(x: &dyn IsEven) {
    x.is_even().log();
}

fn is_even_and_log_box(x: Box<dyn IsEven>) {
    x.is_even().log();
}

struct VecWrapper(Vec<String>);

impl fmt::Display for VecWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.join(", "))
    }
}

impl ops::Deref for VecWrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for VecWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
