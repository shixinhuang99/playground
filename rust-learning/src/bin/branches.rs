fn main() {
    let val = 1;

    if val > 0 {
        println!("{} {}", val, true);
    } else {
        println!("{}", false);
    }

    let val = if val > 0 { 'a' } else { 'b' };

    println!("{}", val);
}
