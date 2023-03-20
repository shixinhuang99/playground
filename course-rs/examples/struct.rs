fn main() {
    let user1 = User {
        email: String::from("foo@bar.com"),
        username: String::from("foo"),
        active: true,
        sign_in_count: 34472,
    };

    println!("{:?}", user1);

    let user2 = User {
        email: String::from("foo2@bar.com"),
        ..user1
    };

    println!("{} {} {}", user1.email, user1.active, user1.sign_in_count);
    // println!("{:?}", user1);
    // println!("{}", user1.username);
    println!("{:?}", user2);
    println!("{}", user2.username);

    println!("{:?}", Foo::A);
    let a = Foo::A;
    println!("{:?}", a);

    let a = Foo::A as u32;
    let b = Foo::B as u32;
    let c = Foo::C as u32;
    println!("{} {} {}", a, b, c);
}

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

#[derive(Debug)]
#[repr(u32)]
enum Foo {
    A = 1001,
    B,
    C,
}
