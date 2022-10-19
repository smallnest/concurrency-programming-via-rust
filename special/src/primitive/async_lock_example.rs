use async_lock::*;
use std::sync::Arc;

pub fn async_lock_mutex() {
    let lock = Arc::new(Mutex::new(0));

    let lock1 = lock.clone();
    smol::block_on(async {
        let mut guard = lock1.lock().await;
        *guard += 1;
    });

    let lock2 = lock.clone();
    smol::block_on(async {
        let guard = lock2.lock().await;
        println!("lock2 {}", *guard);
    });

    
}
