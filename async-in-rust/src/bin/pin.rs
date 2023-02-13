use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;

use futures::executor::block_on;

fn main() {
    let foo_1 = Foo::new("1");

    let foo_2 = Foo::new("2");

    println!(
        "foo_1 -> a: {:?}, b: {:?}",
        Foo::a(foo_1.as_ref()),
        Foo::b(foo_1.as_ref())
    );

    println!(
        "foo_1 -> a: {:?}, b: {:?}",
        Foo::a(foo_2.as_ref()),
        Foo::b(foo_2.as_ref())
    );

    let fut = async { 1 };
    let boxed_fut = Box::pin(fut);
    let res = block_on(excute_unpin_future(boxed_fut));
    println!("{}", res);
}

struct Foo {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Foo {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(Foo {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        });
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe {
            boxed.as_mut().get_unchecked_mut().b = self_ptr;
        }

        boxed
    }

    fn _init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

async fn excute_unpin_future(f: impl Future<Output = i32> + Unpin) -> i32 {
    f.await
}
