use std::ops::Add;
use std::fmt;

struct Counter {
    count: i32,
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= i32::MAX {
            None
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}

fn example1() {
    let mut counter = Counter { count: 0 };

    assert_eq!(Some(1), counter.next());
}

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

fn example2() {
    assert_eq!(Point::new(1, 0) + Point::new(2, 3), Point::new(3, 3))
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Millimeters(u32);

#[derive(PartialEq, Debug, Clone, Copy)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

impl Add<Millimeters> for Meters {
    type Output = Meters;

    fn add(self, rhs: Millimeters) -> Self::Output {
        Meters(self.0 + (rhs.0 / 1000))
    }
}

fn example3() {
    let millimeters = Millimeters(2345);
    let meters = Meters(3);

    assert_eq!(Millimeters(5345), millimeters + meters);
    assert_eq!(Meters(5), meters + millimeters);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn example4() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn example5() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn example6() {
    Point::new(3, 4).outline_print();
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn example7() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
}
