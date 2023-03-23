use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = read_cargo_cfg()?;
    println!("{}", s);

    let arr = [1, 2, 3];
    if let Some(v) = first(&arr) {
        println!("{}", v);
    }

    Ok(())
}

fn read_cargo_cfg() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("Cargo.toml")?.read_to_string(&mut s)?;

    Ok(s)
}

fn first(arr: &[i32]) -> Option<&i32> {
    arr.get(0)
}
