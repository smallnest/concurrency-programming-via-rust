use waitgroup::WaitGroup;

pub fn waitgroup_example() {
    smol::block_on(async {
        let wg = WaitGroup::new();
        for _ in 0..100 {
            let w = wg.worker();
            let _ = smol::spawn(async move {
                // do work
                drop(w); // drop w means task finished
            });
        }

        wg.wait().await;
    })
}

pub fn wg_example() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread::{sleep, spawn};
    use std::time::Duration;
    use wg::WaitGroup;

    let wg = WaitGroup::new();
    let ctr = Arc::new(AtomicUsize::new(0));

    for _ in 0..5 {
        let ctrx = ctr.clone();
        let t_wg = wg.add(1);
        spawn(move || {
            // mock some time consuming task
            sleep(Duration::from_millis(50));
            ctrx.fetch_add(1, Ordering::Relaxed);

            // mock task is finished
            t_wg.done();
        });
    }

    wg.wait();
    assert_eq!(ctr.load(Ordering::Relaxed), 5);
}
