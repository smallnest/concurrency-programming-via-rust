
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

pub fn rwlock_example() {
    let rwlock = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let rwlock = rwlock.clone();
        handles.push(thread::spawn(move || {
            let num = rwlock.read().unwrap();
            println!("num1: {}", num);
        }));
    }

    for _ in 0..10 {
        let rwlock = rwlock.clone();
        handles.push(thread::spawn(move || {
            let mut num = rwlock.write().unwrap();
            *num += 1;
        }));
    }
    for _ in 0..10 {
        let rwlock = rwlock.clone();
        handles.push(thread::spawn(move || {
            let num = rwlock.read().unwrap();
            println!("num2: {}", num);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("RwLock: {}", *rwlock.read().unwrap());
}   