#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use std::rc::Rc;

use futures::{
    executor::block_on,
    future::{BoxFuture, FutureExt},
};

// use async_trait::async_trait;

fn main() {
    let fut = async {
        a().await?;
        b().await?;

        Ok::<(), String>(())
    };

    let res = block_on(fut);
    if let Err(msg) = res {
        println!("{}", msg);
    }

    require_send(foo());
}

async fn a() -> Result<(), String> {
    Ok(())
}

async fn b() -> Result<(), String> {
    Err("error".to_string())
}

#[derive(Default)]
struct NotSend(Rc<()>);

async fn bar() {
    //
}

async fn foo() {
    {
        let _ = NotSend::default();
    }
    bar().await;
}

fn require_send(_: impl Send) {
    //
}

#[allow(dead_code)]
fn recursive() -> BoxFuture<'static, ()> {
    async move {
        recursive().await;
    }
    .boxed()
}

// #[async_trait]
trait Test {
    async fn test();
}
