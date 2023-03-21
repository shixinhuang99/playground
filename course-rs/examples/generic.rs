#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::mem;

fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 2] = [1, 2];
    display_array(arr);

    display_size([0u8; 1]);
    display_size([0u8; 1024]);
    // display_size([0u8; 2048]);
}

fn display_array<T, const N: usize>(arr: [T; N])
where
    T: std::fmt::Debug,
{
    println!("{:?}", arr);
}

fn display_size<T>(val: T)
where
    Assert<{ mem::size_of::<T>() <= 1024 }>: IsTrue,
{
    let size = mem::size_of_val(&val);
    println!("{}", size);
}

struct Assert<const T: bool>;

trait IsTrue {}

impl IsTrue for Assert<true> {}
