pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Option<Guess> {
        if value < 1 || value > 100 {
            return None;
        }
        Some(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
