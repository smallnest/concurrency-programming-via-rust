pub mod others;
pub use others::*;

use std::sync::mpsc;
use std::sync::mpsc::sync_channel;
use std::thread;

pub fn mpsc_example1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn mpsc_example2() {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        println!("Got: {}", j);
    }
}

pub fn mpsc_example3() {
    let (tx, rx) = sync_channel::<i32>(0);
    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        tx.send(53).unwrap();
    });
    rx.recv().unwrap();
}

pub fn mpsc_example4() {
    let (tx, rx) = sync_channel(3);

    for _ in 0..3 {
        let tx = tx.clone();
        // cloned tx dropped within thread
        thread::spawn(move || tx.send("ok").unwrap());
    }

    drop(tx);

    // Unbounded receiver waiting for all senders to complete.
    while let Ok(msg) = rx.recv() {
        println!("{msg}");
    }

    println!("mpsc_example4 completed");
}
