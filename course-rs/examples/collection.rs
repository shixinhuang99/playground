use std::collections::HashMap;

fn main() {
    let m: HashMap<char, i32> = HashMap::from_iter(vec![('a', 1), ('b', 2)]);
    println!("{:?}", m);

    let v = vec![('a', 1), ('b', 2)];
    let m: HashMap<char, i32> = v.into_iter().collect();
    println!("{:?}", m);

    let m: HashMap<Point, i32> =
        HashMap::from_iter(vec![(Point(1, 1), 4), (Point(0, -6), 0)]);
    println!("{:?}", m);
    println!("{}", m.get(&Point(1, 1)).expect("should get 4"));
}

#[derive(Hash, Debug)]
struct Point(i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}
