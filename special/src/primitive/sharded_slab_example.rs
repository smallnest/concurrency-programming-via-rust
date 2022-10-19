use sharded_slab::Pool;
use sharded_slab::Slab;

use std::sync::{Arc, Mutex};

// Slabs provide pre-allocated storage for many instances of a single data type.
// When a large number of values of a single type are required, this can be more efficient than allocating each item individually.
// Since the allocated items are the same size, memory fragmentation is reduced, and creating and removing new items can be very cheap.

pub fn sharded_slab_read() {
    let slab = Arc::new(Slab::new());

    let slab2 = slab.clone();
    let thread2 = std::thread::spawn(move || {
        let key = slab2.insert("hello from thread two").unwrap();
        assert_eq!(slab2.get(key).unwrap(), "hello from thread two");
        key
    });

    let key1 = slab.insert("hello from thread one").unwrap();
    assert_eq!(slab.get(key1).unwrap(), "hello from thread one");

    // Wait for thread 2 to complete.
    let key2 = thread2.join().unwrap();

    // The item inserted by thread 2 remains in the slab.
    assert_eq!(slab.get(key2).unwrap(), "hello from thread two");
}

pub fn sharded_slab_write() {
    let slab = Arc::new(Slab::new());

    let key = slab
        .insert(Mutex::new(String::from("hello world")))
        .unwrap();

    let slab2 = slab.clone();
    let thread2 = std::thread::spawn(move || {
        let hello = slab2.get(key).expect("item missing");
        let mut hello = hello.lock().expect("mutex poisoned");
        *hello = String::from("hello everyone!");
    });

    thread2.join().unwrap();

    let hello = slab.get(key).expect("item missing");
    let hello = hello.lock().expect("mutex poisoned");
    assert_eq!(hello.as_str(), "hello everyone!");
}

pub fn sharded_slab_pool() {
    let pool: Pool<String> = Pool::new();

    let mut guard = pool.create().unwrap();
    let key = guard.key();
    guard.push_str("hello world");

    drop(guard); // release the guard, allowing immutable access.
    assert_eq!(pool.get(key).unwrap(), String::from("hello world"));

    // Mark this entry to be cleared.
    pool.clear(key);
    // The cleared entry is no longer available in the pool
    assert!(pool.get(key).is_none());
}

pub fn slab_example() {
    let mut slab = slab::Slab::new();

    let hello = slab.insert("hello");
    let world = slab.insert("world");

    assert_eq!(slab[hello], "hello");
    assert_eq!(slab[world], "world");

    slab[world] = "earth";
    assert_eq!(slab[world], "earth");
}
