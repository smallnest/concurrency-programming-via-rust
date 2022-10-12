use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicI32, Ordering};
use std::{hint, thread};

pub fn atomic_example() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move|| {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {panic:?}");
    } else {
        println!("atomic result: {}", spinlock.load(Ordering::SeqCst));
    }
}

pub fn atomic_example2() {
    let v = AtomicI32::new(5);

    v.store(100, Ordering::SeqCst);

    println!("atomic load:{}", v.load(Ordering::SeqCst));
    println!("atomic swap:{}",  v.swap(5, Ordering::SeqCst));
    println!("atomic swap:{}",  v.compare_exchange(5,100, Ordering::SeqCst,Ordering::SeqCst).unwrap());
    println!("atomic fetch_add:{}", v.fetch_add(1, Ordering::SeqCst));
    println!("atomic load:{}", v.load(Ordering::SeqCst));
    println!("atomic fetch_sub:{}", v.fetch_sub(1, Ordering::SeqCst));
    println!("atomic load:{}", v.load(Ordering::SeqCst));
    println!("atomic fetch_and:{}", v.fetch_and(1, Ordering::SeqCst));
    println!("atomic load:{}", v.load(Ordering::SeqCst));
    println!("atomic fetch_or:{}", v.fetch_or(1, Ordering::SeqCst));
    println!("atomic load:{}", v.load(Ordering::SeqCst));
    println!("atomic fetch_xor:{}", v.fetch_xor(1, Ordering::SeqCst));
    println!("atomic load:{}", v.load(Ordering::SeqCst));
    println!("atomic fetch_nand:{}", v.fetch_nand(1, Ordering::SeqCst));
    println!("atomic load:{}", v.load(Ordering::SeqCst));

}