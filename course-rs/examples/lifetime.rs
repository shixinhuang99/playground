fn main() {
    let s = longest("aa", "b", Some("aa is longest"));
    println!("{}", s);
}

fn longest<'a, T>(x: &'a str, y: &'a str, ann: Option<T>) -> &'a str
where
    T: std::fmt::Display,
{
    if let Some(ann) = ann {
        println!("{}", ann);
    }
    if x.len() > y.len() { x } else { y }
}
