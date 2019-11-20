use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let second_handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    second_handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("Single mutex: m = {:?}", m);

    let multi_counter = Arc::new(Mutex::new(0));
    let mut multi_handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&multi_counter);
        let multi_handle = thread::spawn(move || {
            let mut num = c.lock().unwrap();

            *num += 1;
        });
        multi_handles.push(multi_handle);
    }

    for multi_handle in multi_handles {
        multi_handle.join().unwrap();
    }

    println!("Multi mutex: counter = {}", *multi_counter.lock().unwrap());
}
