use futures::future::join_all;
use singleflight_async::SingleFlight;
use std::sync::Arc;

use async_singleflight::Group;

pub fn singleflight_example() {
    smol::block_on(async {
        let group = SingleFlight::new();
        let mut futures = Vec::new();
        for _ in 0..10 {
            futures.push(group.work("key", || async {
                println!("will sleep to simulate async task");
                smol::Timer::after(std::time::Duration::from_millis(100)).await;
                println!("real task done");
                "my-result"
            }));
        }

        for fut in futures.into_iter() {
            assert_eq!(fut.await, "my-result");
            println!("task finished");
        }
    });
}

const RES: usize = 7;

async fn expensive_fn() -> Result<usize, ()> {
    smol::Timer::after(std::time::Duration::from_millis(100)).await;
    Ok(RES)
}

pub fn async_singleflight_example() {
    smol::block_on(async {
        let g = Arc::new(Group::<_, ()>::new());
        let mut handlers = Vec::new();
        for _ in 0..10 {
            let g = g.clone();
            handlers.push(smol::spawn(async move {
                let res = g.work("key", expensive_fn()).await.0;
                let r = res.unwrap();
                println!("{}", r);
            }));
            
        }

        join_all(handlers).await;
    });
}
