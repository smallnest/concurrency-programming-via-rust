use futures::future::Future;
use futures::task::{Context, Poll, AtomicWaker};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::pin::Pin;

struct Inner {
    waker: AtomicWaker,
    set: AtomicBool,
}

#[derive(Clone)]
struct Flag(Arc<Inner>);

impl Flag {
    pub fn new() -> Self {
        Flag(Arc::new(Inner {
            waker: AtomicWaker::new(),
            set: AtomicBool::new(false),
        }))
    }

    pub fn signal(&self) {
        self.0.set.store(true, Relaxed);
        self.0.waker.wake();
    }
}

impl Future for Flag {
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool> {
        // quick check to avoid registration if already done.
        if self.0.set.load(Relaxed) {
            return Poll::Ready(true);
        }

        self.0.waker.register(cx.waker());

        // Need to check condition **after** `register` to avoid a race
        // condition that would result in lost notifications.
        if self.0.set.load(Relaxed) {
            Poll::Ready(true)
        } else {
            Poll::Pending
        }
    }
}

// extraced from futures::task::AtomicWaker
pub fn atomic_waker_example() {
    smol::block_on(async {
        let flag = Flag::new();
        let flag2 = flag.clone();

        smol::spawn(async move {
            smol::Timer::after(std::time::Duration::from_secs(1)).await;
            flag2.signal();
        })
        .detach();

        println!("Waiting for flag: {}", flag.await);
    });
}