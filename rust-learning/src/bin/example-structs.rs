#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rectangle is {:?}", rect1);
    println!("rectangle is {:#?}", rect1);
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    let rect3 = Rectangle {
        width: 70,
        height: 110,
    };
    println!("can hold: {}", rect1.can_hold(&rect2));
    println!("can hold: {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(30);
    println!("square is: {:#?}", sq);
}
