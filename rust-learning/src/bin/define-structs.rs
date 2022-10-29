fn main() {
    let mut user = User {
        username: String::from("luna"),
        email: String::from("luna@foo.com"),
        active: true,
        sign_in_count: 7484,
    };
    user.active = false;
    println!("{} {} {}", user.username, user.email, user.sign_in_count);

    let _user2 = build_user(String::from("anna"), String::from("anna@foo.com"));

    let _user3 = User {
        username: String::from("lucy"),
        email: String::from("lucy@foo.com"),
        .._user2
    };

    let _user4 = User { .._user2 };
    // moved
    // _user2.email;

    let black = build_color(0, 0, 0);
    println!("rgb: #{}{}{}", black.0, black.1, black.2);

    let _subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);

fn build_color(r: i32, g: i32, b: i32) -> Color {
    Color(r, g, b)
}

struct AlwaysEqual;
