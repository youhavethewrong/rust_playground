extern crate junk;

fn main() {
    println!("Main screen turn on!");

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + 1);

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
