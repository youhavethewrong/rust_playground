use std::env;

fn main() {
    println!("Main screen turn on.\n");
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("in file {}", filename);
}
