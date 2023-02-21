use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::{select, spawn, sync::oneshot};

#[tokio::main]
async fn main() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    spawn(async {
        select! {
            val = some_operation() => {
                tx1.send(val).expect("tx1 should be send successfully");
            }
            _ = tx1.closed() => {
                println!("tx1 closed");
            }
        }
    });

    spawn(async {
        tx2.send("two").expect("tx2 should be send successfully");
    });

    select! {
        val = rx1 => {
            println!("rx1 {:?}", val.expect("rx1 should be recived a string"));
        }
        val = rx2 => {
            println!("rx2 {:?}", val.expect("rx2 should be recived a string"));
        }
    }

    println!("select done");

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    spawn(async {
        tx1.send("one").expect("tx1 should be send successfully");
    });

    spawn(async {
        tx2.send("two").expect("tx2 should be send successfully");
    });

    MySelect { rx1, rx2 }.await;

    println!("my select done");
}

async fn some_operation() -> String {
    println!("some operation");

    "some operation".to_string()
}

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 {:?}", val.expect("rx1 should be recived a string"));
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 {:?}", val.expect("rx2 should be recived a string"));
            return Poll::Ready(());
        }

        Poll::Pending
    }
}
