extern crate junk;

use junk::CustomSmartPointer;
use junk::List::{Cons, Nil};
use std::mem::drop;

fn main() {
    println!("Main screen turn on!");

    let x = Box::new(27);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + 1);

    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("Woah I have {} in a Box!", x);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    let e = CustomSmartPointer {
        data: String::from("wow cool"),
    };
    println!("CustomSmartPointers created.");
    drop(e);
    println!("^ this was dropped before the end of main.");
}
