use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam::utils::Backoff;
use crossbeam::utils::CachePadded;

pub fn backoff_example() {
    fn spin_wait(ready: &AtomicBool) {
        let backoff = Backoff::new();
        while !ready.load(SeqCst) {
            backoff.snooze();
        }
    }

    let ready = Arc::new(AtomicBool::new(false));
    let ready2 = ready.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        ready2.store(true, SeqCst);
    });

    assert_eq!(ready.load(SeqCst), false);
    spin_wait(&ready);
    assert_eq!(ready.load(SeqCst), true);
}

pub fn cachepadded_example() {
    let array = [CachePadded::new(1i8), CachePadded::new(2i8)];
    let addr1 = &*array[0] as *const i8 as usize;
    let addr2 = &*array[1] as *const i8 as usize;

    assert!(addr2 - addr1 >= 128);
    assert_eq!(addr1 % 128, 0);
    assert_eq!(addr2 % 128, 0);
}

pub fn scope_example() {
    let v = vec![1, 2, 3];

    crossbeam::thread::scope(|s| {
        s.spawn(|_| {
            println!("A child thread borrowing `v`: {:?}", v);
        });
    })
    .unwrap();

    println!("scope_exampleg `v`: {:?}", v);
}
