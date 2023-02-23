fn main() {
    let a = 1;
    let b = add_one(a);

    println!("{}, {}", a, b);

    let (a, .., b, _) = (1, 2, 3, 4);
    println!("{}, {}", a, b);

    let (mut x, mut y) = (1, 2);
    println!("{}, {}", x, y);
    (x, y) = (3, 4);
    println!("{}, {}", x, y);

    let mut foo = 0;
    println!("{}", foo);

    let foo1 = Foo { foo: 3 };

    Foo { foo } = foo1;

    println!("{}", foo);

    println!("{}, {}", C_A, PI);

    println!("{} {}{}", CONST_STR_1, CONST_STR_2, CONST_STR_3);

    let a: i32 = "111".parse().expect("should parse into i32");
    println!("{}", a);

    let a = 1;
    let b = 0xf;
    let c = 0o7;
    let d = 0b1011001;
    let e = b'F';
    println!("{}, {}, {}, {}, {}", a, b, c, d, e);

    let a: u8 = 255;
    let b = match a.checked_add(1) {
        Some(n) => n,
        None => {
            eprintln!("overflow");
            a
        }
    };
    println!("{}", b);

    let a = 0.1_f64;
    let b = 0.2_f64;
    let c = 0.3_f64;

    println!("{} {}", a + b, c);
    println!("{}", cmp_f64(a + b, c))
}

fn add_one(mut num: i32) -> i32 {
    num += 1;
    num
}

struct Foo {
    foo: i32,
}

const C_A: i32 = 1024 * 4;
const PI: f32 = 3.141592654;
const CONST_STR_1: &'static str = "hello";
const CONST_STR_2: &str = "world";
const CONST_STR_3: &'_ str = "!";

fn cmp_f64(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}
