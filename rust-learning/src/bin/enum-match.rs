#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn print_ip_kind(ip_kind: &IpAddrKind) {
    println!("ip kind: {:?}", ip_kind);
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { _x: i32, _y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn println(&self) {
        println!("messge: {:?}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    print_ip_kind(&four);
    print_ip_kind(&six);
    print_ip_kind(&IpAddrKind::V4);
    print_ip_kind(&IpAddrKind::V6);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    let msg_quit = Message::Quit;
    let msg_move = Message::Move { _x: 1, _y: 10 };
    let msg_write = Message::Write(String::from("hello"));
    let msg_change_color = Message::ChangeColor(255, 255, 255);

    for msg in [msg_quit, msg_move, msg_write, msg_change_color] {
        msg.println();
    }

    let some_number = Some(5);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;
    println!("{:?} {:?} {:?}", some_number, some_char, absent_number);

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter_alabama = Coin::Quarter(UsState::Alabama);
    let quarter_alska = Coin::Quarter(UsState::Alska);

    for coin in [penny, nickel, dime, quarter_alabama, quarter_alska] {
        println!("coin: {}", value_in_cents(&coin));
    }

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max: {}", max);
    } else {
        println!("none");
    }
}
