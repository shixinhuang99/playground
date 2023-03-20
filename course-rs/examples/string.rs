fn main() {
    let byte_escape = "foo \x52\x75\x73\x74\x3f";

    println!("{}", byte_escape);

    let unicode_codepoint = "\u{211d}";

    println!("{}", unicode_codepoint);

    let long_string = "f
                       o ->\
                       <- o";
    println!("{}", long_string);

    let raw_str = r"foo \x52\x75\x73\x74\x3f";
    println!("{}", raw_str);

    let raw_str = r#"foo "\x52\x75\x73\x74\x3f""#;
    println!("{}", raw_str);

    let raw_str = r###""""foo""""###;
    println!("{}", raw_str)
}
