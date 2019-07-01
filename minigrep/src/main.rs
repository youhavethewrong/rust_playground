use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Main screen turn on.\n");
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("in file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong with the file");

    println!("With text:\n{}", contents);
}
