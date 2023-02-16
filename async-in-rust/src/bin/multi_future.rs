use futures::{
    executor::block_on,
    future::{Fuse, FusedFuture, FutureExt, TryFutureExt},
    join, pin_mut, select, try_join,
};

fn main() {
    block_on(async_main());
}

async fn async_main() {
    join!(a(), b());

    match try_join!(c().map_err(|_| "error".to_string()), d()) {
        Ok((a, b)) => println!("{}", a + b),
        Err(msg) => eprintln!("{}", msg),
    }

    println!("\n--select--\n");

    select! {
        c_res = c().fuse() => {
            match c_res {
                Ok(n) => println!("{}", n),
                Err(_) => println!("fut_c error"),
            }
        },
        d_res = d().fuse() => {
            match d_res {
                Ok(n) => println!("{}", n),
                Err(msg) => eprintln!("{}", msg),
            }
        },
        e_res = e(0.1).fuse() => println!("{}", e_res),
    }

    println!("\n--loop select--\n");

    let fut_c = c().fuse();
    let fut_d = d().fuse();
    let fut_e = e(1.1).fuse();

    pin_mut!(fut_c, fut_d, fut_e);

    loop {
        select! {
            c_res = fut_c => {
                match c_res {
                    Ok(n) => println!("{}", n),
                    Err(_) => println!("fut_c error"),
                }
            },
            d_res = fut_d => {
                match d_res {
                    Ok(n) => println!("{}", n),
                    Err(msg) => eprintln!("{}", msg),
                }
            },
            e_res = fut_e => println!("{}", e_res),
            complete => {
                println!("complete");
                break;
            },
            default => println!("default"),
        }
    }

    println!("\n--select default--\n");

    let mut fut_pending = futures::future::pending().fuse();
    loop {
        select! {
            () = fut_pending => println!("fut_pending"),
            complete => {
                println!("complete");
                break;
            },
            default => {
                println!("default");
                break;
            }
        }
    }

    println!("\n--Fuse::terminated--\n");

    let fut_e = e(2.1).fuse();
    let get_new_fut = Fuse::terminated();
    pin_mut!(fut_e, get_new_fut);
    loop {
        select! {
            e_res = fut_e => {
                println!("{}", e_res);
                if get_new_fut.is_terminated() {
                    get_new_fut.set(e(3.1).fuse());
                }
            },
            new_fut_res = get_new_fut => {
                println!("{}", new_fut_res);
            },
            complete => break,
        }
    }
}

async fn a() {
    println!("a");
}

async fn b() {
    println!("b");
}

async fn c() -> Result<i32, ()> {
    Err(())
    // Ok(2)
}

async fn d() -> Result<i32, String> {
    Ok(1)
}

async fn e(num: f32) -> f32 {
    num
}
