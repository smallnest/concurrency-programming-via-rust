use futures::pin_mut;
use futures::poll;
use std::sync::Arc;

pub fn tokio_semaphore_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(3));
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

pub fn async_weighted_semaphore_example() {
    smol::block_on(async {
        let sem = async_weighted_semaphore::Semaphore::new(1);
        let a = sem.acquire(2);
        let b = sem.acquire(1);
        pin_mut!(a);
        pin_mut!(b);
        assert!(poll!(&mut a).is_pending());
        assert!(poll!(&mut b).is_pending());

        sem.release(1);
        assert!(poll!(&mut a).is_ready());
        assert!(poll!(&mut b).is_ready());
    });
}

pub fn async_lock_semaphore() {
    let s = Arc::new(async_lock::Semaphore::new(2));

    let _g1 = s.try_acquire_arc().unwrap();
    let g2 = s.try_acquire_arc().unwrap();

    assert!(s.try_acquire_arc().is_none());
    drop(g2);
    assert!(s.try_acquire_arc().is_some());
}
