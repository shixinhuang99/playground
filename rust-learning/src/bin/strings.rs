fn main() {
    let _s = String::new();
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");
    let _s = String::from("😅");
    let mut s = String::from("foo");
    s.push_str("bar");
    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{}", s3);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{} {}", s, s1);
    let hello = "Здравствуйте";
    println!("{}", hello.len());
    let s = &hello[0..4];
    println!("{} {}", hello, s);
    let hello = "नमस्ते";
    for ch in hello.chars() {
        println!("{}", ch);
    }
    for byte in hello.bytes() {
        println!("{}", byte);
    }
}
