use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("search \"{}\" in file \"{}\".", query, file_path);

    let content = fs::read_to_string(file_path)?;

    println!("file content:\n{}", content);

    Ok(())
}
