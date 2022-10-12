use std::cell::SyncUnsafeCell;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.
// Unlike Rc<T>, Arc<T> uses atomic operations for its reference counting. This means that it is thread-safe.

pub fn arc_example() {
    let five = Arc::new(5);

    for _ in 0..10 {
        let five = Arc::clone(&five);

        thread::spawn(move || {
            println!("{five:?}");
        });
    }
}

pub fn arc_example2() {
    let val = Arc::new(AtomicUsize::new(5));

    for _ in 0..10 {
        let val = Arc::clone(&val);

        thread::spawn(move || {
            let v = val.fetch_add(1, Ordering::SeqCst);
            println!("{v:?}");
        });
    }
}

pub fn arc_example3() {
    let val = Arc::new(SyncUnsafeCell::new(5));
    let val2 = &val.clone();

    let mut handles = vec![];

    for _ in 0..10 {
        let val = Arc::clone(&val);

        let handle = thread::spawn(move || {
            let v = val.deref().get();
            unsafe {*v += *v};
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    unsafe {
        let v = val2.deref().get().read();
        println!("SyncUnsafeCell: {:?}",  v);
    }
    
    
}

