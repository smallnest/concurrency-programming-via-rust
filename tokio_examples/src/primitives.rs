use std::sync::Arc;
use tokio::sync::Barrier;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

use tokio::sync::Notify;
use tokio::sync::{Semaphore, TryAcquireError};

pub fn barrier_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut handles = Vec::with_capacity(10);
        let barrier = Arc::new(Barrier::new(10));
        for _ in 0..10 {
            let c = barrier.clone();
            // The same messages will be printed together.
            // You will NOT see any interleaving.
            handles.push(tokio::spawn(async move {
                println!("before wait");
                let wait_result = c.wait().await;
                println!("after wait");
                wait_result
            }));
        }

        // Will not resolve until all "after wait" messages have been printed
        let mut num_leaders = 0;
        for handle in handles {
            let wait_result = handle.await.unwrap();
            if wait_result.is_leader() {
                num_leaders += 1;
            }
        }

        // Exactly one barrier will resolve as the "leader"
        assert_eq!(num_leaders, 1);
    });
}

pub fn mutex_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let data1 = Arc::new(Mutex::new(0));
        let data2 = Arc::clone(&data1);

        tokio::spawn(async move {
            let mut lock = data2.lock().await;
            *lock += 1;
        });

        let mut lock = data1.lock().await;
        *lock += 1;
    });
}

pub fn rwlock_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let lock = RwLock::new(5);

        // many reader locks can be held at once
        {
            let r1 = lock.read().await;
            let r2 = lock.read().await;
            assert_eq!(*r1, 5);
            assert_eq!(*r2, 5);
        } // read locks are dropped at this point

        // only one write lock may be held, however
        {
            let mut w = lock.write().await;
            *w += 1;
            assert_eq!(*w, 6);
        } // write lock is dropped here
    });
}

pub fn semaphore_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let semaphore = Semaphore::new(3);

        let _a_permit = semaphore.acquire().await.unwrap();
        let _two_permits = semaphore.acquire_many(2).await.unwrap();

        assert_eq!(semaphore.available_permits(), 0);

        let permit_attempt = semaphore.try_acquire();
        assert_eq!(permit_attempt.err(), Some(TryAcquireError::NoPermits));
    });
}

pub fn semaphore_example2() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let semaphore = Arc::new(Semaphore::new(3));
        let mut join_handles = Vec::new();

        for _ in 0..5 {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            join_handles.push(tokio::spawn(async move {
                // perform task...
                // explicitly own `permit` in the task
                drop(permit);
            }));
        }

        for handle in join_handles {
            handle.await.unwrap();
        }
    });
}

pub fn notify_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let notify = Arc::new(Notify::new());
        let notify2 = notify.clone();

        let handle = tokio::spawn(async move {
            notify2.notified().await;
            println!("received notification");
        });

        println!("sending notification");
        notify.notify_one();

        // Wait for task to receive notification.
        handle.await.unwrap();
    });
}

pub fn notify_example2() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let notify = Arc::new(Notify::new());
        let notify2 = notify.clone();

        let notified1 = notify.notified();
        let notified2 = notify.notified();

        let _handle = tokio::spawn(async move {
            println!("sending notifications");
            notify2.notify_waiters();
        });

        notified1.await;
        notified2.await;
        println!("received notifications");
    });
}
