use std::ops::{Deref, DerefMut};

fn main() {
    let x = 5;
    let y = &x;
    let y2 = Box::new(x);
    let y3 = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *y2);
    assert_eq!(5, *y3);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let mut m2 = MyBox::new(String::from("Rust"));
    m2.0.push('!');
    hello(&m2);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
