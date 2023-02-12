fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s2 = s.clone();
    assert_eq!(s, s2);

    let x = 5;
    let y = x;
    assert_eq!(x, y);

    let some_str = String::from("str");
    takes_ownership(some_str);
    // println!("{}", some_str);

    let z = 5;
    makes_copy(z);
    println!("{}", z);

    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);

    let s = String::from("hello");
    let (s, len) = calculate_length(s);
    println!("value: {}, length: {}", s, len);

    let len = calculate_length_v2(&s);
    println!("{}", len);

    let mut s = String::from("hello");
    let len = s.len();
    change(&mut s);
    assert_eq!(len + 3, s.len());

    let mut s = String::from("a");
    {
        let r1 = &mut s;
        r1.push('b');
    }
    let r2 = &mut s;
    r2.push('c');
    assert_eq!(r2.len(), 3);
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(mut int: i32) {
    int += 1;
    println!("{}", int);
}

fn gives_ownership() -> String {
    let str = String::from("foo");
    str
}

fn takes_and_gives_back(str: String) -> String {
    str
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length_v2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("foo");
}
