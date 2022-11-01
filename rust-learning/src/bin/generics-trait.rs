use std::fmt::{Display, Debug};
use std::clone::Clone;
use aggregator::{Summary, Tweet, ToString};

fn main() {
    let list1 = [1, 2, 3, 4];
    let largest = get_largest(&list1);
    println!("largest1: {}", largest);
    println!("list1: {:?}", list1);

    let list2 = [1, 2, 3, 4];
    match get_largest_v2(&list2) {
        Some(largest) => {
            println!("largest2: {}", largest);
        }
        None => (),
    };
    println!("list2: {:?}", list2);

    let p1 = Point::new(5, 6.0);
    println!("({}, {})", p1.x(), p1.y());

    let p2 = Point::new(4.0, 8.0);
    println!("distance: {}", p2.distance_from_origin());

    let p3 = p1.mixup(p2);
    println!("({}, {})", p3.x(), p3.y());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probaly already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let weibo = Weibo {
        author: String::from("foo"),
    };
    println!("{}", weibo.summarize());

    notify(&tweet);
    notify_v2(&weibo);
    // notify_v3(&tweet, &weibo);
    // notify_v4(&tweet, &tweet);

    Pair::new(4, 5).cmp_display();

    println!("to_string: {}", tweet.to_string());
}

fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn get_largest_v2<T: PartialOrd>(list: &[T]) -> Option<&T> {
    let mut largest = list.get(0)?;
    for i in list {
        if i > largest {
            largest = i
        }
    }
    Some(largest)
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct Weibo {
    author: String,
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound
fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn _notify_v3(_item1: &impl Summary, _item2: &impl Summary) {
    todo!();
}

// trait bound
fn _notify_v4<T: Summary>(_item1: &T, _item2: &T) {
    todo!();
}

// multiple trait
fn _notify_v5(_item: &(impl Summary + Display)) {
    todo!();
}

// multiple trait bound
fn _notify_v6<T: Summary + Display>(_item: &T) {
    todo!();
}

fn _some_function<T: Display + Debug, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    todo!();
}

// where
fn _some_function_v2<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    todo!();
}

fn _returns_summerizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probaly already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        let (key, value) = if self.x > self.y {
            ('x', &self.x)
        } else {
            ('y', &self.y)
        };
        println!("The largest number is {} = {}", key, value);
    }
}
