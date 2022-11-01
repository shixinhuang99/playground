use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = "hello.txt";
    match File::open(file_name) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opeing the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opeing the file: {:?}", other_error)
            }
        },
    };
    let uesr_name = read_username_from_file_v4(file_name)?;
    match last_char_of_first_line(&uesr_name) {
        Some(last_char) => {
            println!("last char: {}", last_char);
            Ok(())
        }
        None => panic!("failed"),
    }
}

fn _read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let mut f = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn _read_username_from_file_v2(file_name: &str) -> Result<String, io::Error> {
    let mut f = File::open(file_name)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn _read_username_from_file_v3(file_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v4(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
