use std::fmt::Display;

struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

fn calculate_average(list: &Vec<i32>) -> f64 {
    let total: i32 = list.iter().sum();
    total as f64 / list.len() as f64
}

impl AveragedCollection {
    pub fn new(list: Vec<i32>) -> AveragedCollection {
        let average = calculate_average(&list);
        AveragedCollection { list, average }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        self.average = calculate_average(&self.list);
    }
}

impl Display for AveragedCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.average)
    }
}

fn main() {
    let mut foo = AveragedCollection::new(vec![1, 2, 3]);

    println!("{}", foo.average());

    foo.add(75);

    println!("{}", foo);

    foo.remove();

    println!("{}", foo);
}
