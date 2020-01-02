fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let d = 98_222;
    let h = 0xff;
    let o = 0o77;
    let b = 0b1111_0000;
    let by = b'A';
    println!("Integer literals: {}, {}, {}, {}, and {}", d, h, o, b, by);

    let y = 2.0;
    let z: f32 = 3.0;
    println!("Floating-point literals: {}, {}", y, z);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!(
        "Basic numeric ops: {}, {}, {}, {}, {}",
        sum, difference, product, quotient, remainder
    );

    let t = true;
    let f: bool = false;
    println!("True is {} and False is {}.", t, f);

    let c = 'c';
    let a = 'a';
    let stupid_emoji = '🌟';
    println!("Chars: {}, {}, {}", c, a, stupid_emoji);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (d, e, f) = tup;
    println!("The value of a tuple is: {}, {}, {}, {:?}", d, e, f, tup);
    println!(
        "Can also access tuple parts directly: {}, {}, {}",
        tup.0, tup.1, tup.2
    );

    let arr = [1, 2, 3, 4, 5];
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    println!("Numbers and months: {:?}, {:?}", arr, months);
    println!(
        "Access them in the way you expect: {}, {}, {}",
        arr[3], months[2], months[11]
    );

    another_function();
}

fn another_function() {
    println!("Another function is called.");
}
