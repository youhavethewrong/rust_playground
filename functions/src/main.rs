use std::time::Instant;

fn main() {
    println!("Hello, functions!");

    let x = 141;
    let y = {
        let x = 21;
        x + 1
    };
    let z = if lucky_number() != 0 {
        lucky_number()
    } else {
        27
    };
    println!("Z should be {}", z);

    another_function(x, y);
    println!("The value of five() is {}.", five());
    println!("The value of plus_one(37) is {}.", plus_one(37));

    let n = lucky_number();
    if n < 5 {
        println!("Lucky number {} is less than 5!", n);
    } else if n == 5 {
        println!("Lucky number {} is exactly 5!", n);
    } else {
        println!("Lucky number {} is greater than 5!", n);
    }

    let mut i = 3;
    while i > 0 {
        println!("{}!", i);
        i = i - 1
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Array 'a' contains element {}.", element);
    }

    for w in (0..4).rev() {
        println!("{}!", w);
    }

    println!("72.0 Fahrenheit is {} Celcius.", f_to_c(72.0));
    println!("34.7 Celcius is {} Fahrenheit.", c_to_f(34.7));

    let before_r = Instant::now();
    println!("The 30th fibonacci number is: {}", recursive_fib(29));
    println!("Naive recursive took {:?}.", Instant::now() - before_r);
}

fn another_function(x: i32, y: i32) {
    println!("Yo, I'm another function and the value of my parameter 'x' is {} while my parameter 'y' is {}.", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn lucky_number() -> i32 {
    five() // feeling lucky?
}

fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn recursive_fib(n: u64) -> u64 {
    if n < 2 {
        return 1;
    }
    recursive_fib(n - 1) + recursive_fib(n - 2)
}
