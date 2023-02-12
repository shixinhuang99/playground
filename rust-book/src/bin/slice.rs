fn main() {
    let mut _s = String::from("hello world");
    let word = first_word_v2(&_s);
    // s.clear();
    println!("word: {}", word);

    let _s2 = "hello world";
    let word2 = first_word_v2(_s2);
    println!("word2: {}", word2);

    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[..2];
    assert_eq!(arr_slice, [1, 2]);
}

fn _first_word(str: &String) -> usize {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}

fn first_word_v2(str: &str) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }
    &str[..]
}
