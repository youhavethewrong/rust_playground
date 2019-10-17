extern crate junk;

use junk::CustomSmartPointer;
use junk::List::{Cons, Nil};
use std::mem::drop;
use std::rc::Rc;

fn main() {
    println!("Main screen turn on!");

    let x = Box::new(27);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + 1);

    for val in v1_iter {
        println!("Got: {}", val);
    }
    println!("Woah I have {} in a Box!", x);

    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));

    let m = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!(
        "Reference count after creating m = {}",
        Rc::strong_count(&m)
    );
    let n = Cons(3, Rc::clone(&m));
    println!(
        "Reference count after creating n = {}",
        Rc::strong_count(&m)
    );
    {
        let o = Cons(4, Rc::clone(&m));
        println!(
            "Reference count after creating o = {}",
            Rc::strong_count(&m)
        );
    }
    println!(
        "Reference count after o goes out of scope = {}",
        Rc::strong_count(&m)
    );

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
