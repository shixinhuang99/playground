use std::future::Future;

use futures::executor::block_on;
use futures::{channel::oneshot, stream, StreamExt};

fn main() {
    let a = &1;
    let b = block_on(double(a));
    println!("{}, {}", a, b);

    let c = block_on(wrap_in_async_block());
    println!("{}", c);

    block_on(blocks());

    block_on(move_block());

    block_on(send_recv());
}

async fn double(x: &u8) -> u8 {
    *x * 2
}

fn wrap_in_async_block() -> impl Future<Output = u8> {
    async {
        let x = 5;
        double(&x).await
    }
}

async fn blocks() {
    let str = "foo".to_string();

    let f_1 = async {
        println!("{}", str);
    };

    let f_2 = async {
        println!("{}", str);
    };

    futures::join!(f_1, f_2);
}

fn move_block() -> impl Future<Output = ()> {
    let mut str = "foo".to_string();

    async move {
        str.push_str(" bar");
        println!("{}", str);
    }
}

async fn send_recv() {
    let size: usize = 3;
    let (tx1, rx1) = oneshot::channel::<i32>();
    let (tx2, rx2) = oneshot::channel::<i32>();
    let (tx3, rx3) = oneshot::channel::<i32>();

    let fut = stream::iter(vec![rx1, rx2, rx3]).for_each_concurrent(
        size,
        |rx| async move {
            if let Ok(f) = rx.await {
                println!("{}", f)
            }
        },
    );

    tx1.send(1).unwrap();
    tx2.send(2).unwrap();
    tx3.send(3).unwrap();

    fut.await;
}
