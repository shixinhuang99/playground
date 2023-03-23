use self::utils::{display_name, something};

fn main() {
    display_name();

    let a = constants::A + constants::B;
    println!("{}", a);

    let a = something();
    println!("{}", a);
}

mod utils {
    use super::constants::{A, B};

    /// moduel name
    const NAME: &str = "FOO";

    /// display the [`NAME`]
    pub fn display_name() {
        println!("{}", NAME);
    }

    /// return [`A`] + [`B`]
    pub fn something() -> i32 {
        A + B
    }
}

mod constants {
    pub const A: i32 = 0;
    pub const B: i32 = 1;
}
