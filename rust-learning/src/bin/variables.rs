const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    println!("{THREE_HOURS_IN_SECONDS}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value fo x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let _pi = 3.14;

    let floored = 3 / 2;
    println!("{floored}");

    let emoji = '😅';
    println!("{emoji}");

    {
        let tuple = (500, 6.4, 'a');
        let (x, y, z) = tuple;
        assert_eq!(x, tuple.0);
        assert_eq!(y, tuple.1);
        assert_eq!(z, tuple.2);

        let _unit_tuple = ();
    }

    {
        let _array: [i32; 4] = [1, 2, 3, -4];
        let _array = [0; 2];
    }
}
