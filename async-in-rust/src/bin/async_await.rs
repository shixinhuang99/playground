use std::thread;
use std::time::Duration;

use futures::{executor::block_on, join};

async fn foo() {
    println!("foo start");

    baz().await;

    println!("foo end")
}

async fn bar() {
    println!("bar");
}

async fn baz() {
    let handler = thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        println!("baz");
    });

    handler.join().unwrap();
}

async fn async_main() {
    join!(foo(), bar());
}

fn main() {
    block_on(async_main());
}
