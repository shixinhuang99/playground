fn main() {
    let _c = CustomSmartPointer::new("my stuff");
    let _d = CustomSmartPointer::new("other stuff");
    drop(_c);
    println!("CustomSmartPointer created.");
}

struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(str: &str) -> CustomSmartPointer {
        CustomSmartPointer {
            data: String::from(str),
        }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
