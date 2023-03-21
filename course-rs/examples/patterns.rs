fn main() {
    Message::new_hello(5).example();
    Message::new_hello(11).example();
    Message::new_hello(1).example();

    let msg @ Message::Hello { id: msg_id } = Message::new_hello(2);
    println!("msg_id: {}", msg_id);
    println!("{:?}", msg);

    if let _msg @ Message::Hello { id: 2 } = msg {
        println!("this id of msg is 2");
    }

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => (),
    }
}

#[derive(Debug)]
enum Message {
    Hello { id: i32 },
}

impl Message {
    fn new_hello(id: i32) -> Self {
        Message::Hello { id }
    }

    fn example(&self) {
        match self {
            Message::Hello { id: id @ 3..=7 } => {
                println!("id is {} and in (3, 7]", id);
            }
            Message::Hello { id: 10..=12 } => {
                println!("id in (10, 12]");
            }
            Message::Hello { id } => {
                println!("id is {}", id);
            }
        };
    }
}
