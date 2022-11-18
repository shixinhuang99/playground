use rust_learning::myvec;
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let v = myvec![1, 2, 3];

    println!("{:?}", v);

    Pancakes::hello_macro();
}
