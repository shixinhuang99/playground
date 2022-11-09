use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));

    let _c = Cons(4, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));

    drop(_c);
    println!("{}", Rc::strong_count(&a));
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
