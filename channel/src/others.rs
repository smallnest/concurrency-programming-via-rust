use crossfire::mpmc;
use crossfire::mpsc;
use flume;

use std::thread;

pub fn crossfire_mpsc() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (tx, rx) = mpsc::bounded_future_both::<i32>(100);

    rt.block_on(async move {
        tokio::spawn(async move {
            for i in 0i32..10 {
                let _ = tx.send(i).await;
                println!("sent {}", i);
            }
        });

        loop {
            if let Ok(_i) = rx.recv().await {
                println!("recv {}", _i);
            } else {
                println!("rx closed");
                break;
            }
        }
    });
}

pub fn crossfire_mpmc() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (tx, rx) = mpmc::bounded_future_both::<i32>(100);

    rt.block_on(async move {
        let mut sender_handles = vec![];

        for _ in 0..4 {
            let tx = tx.clone();
            let handle = tokio::spawn(async move {
                for i in 0i32..10 {
                    let _ = tx.send(i).await;
                    println!("sent {}", i);
                }
            });
            sender_handles.push(handle);
        }

        let mut handles = vec![];
        for i in 0..4 {
            let rx = rx.clone();
            let handle = tokio::spawn(async move {
                loop {
                    if let Ok(_i) = rx.recv().await {
                        println!("thread {} recv {}", i, _i);
                    } else {
                        println!("rx closed");
                        break;
                    }
                }
            });
            handles.push(handle);
        }

        for handle in sender_handles {
            handle.await.unwrap();
        }
        drop(tx);

        for handle in handles {
            handle.await.unwrap();
        }
    });
}

// rx can't aware tx close
pub fn atomic_mpmc() {
    // let (tx, rx) = channel::<i32>(10);

    // let mut sender_handles = vec![];
    // for v in 0..4 {
    //     let tx = tx.clone();
    //     let handle = thread::spawn(move || {
    //         for i in 0i32..10 {
    //             if i % 5 != v {
    //                 continue;
    //             }
    //             let _ = tx.send(i).unwrap();
    //             println!("thread {} atomic_mpmc sent {}", v, i);
    //         }
    //     });

    //     sender_handles.push(handle);
    // }

    // let mut handles = vec![];
    // for i in 0..4 {
    //     let rx = rx.clone();
    //     let handle = thread::spawn(move || loop {
    //         if let Ok(_i) = rx.recv() {
    //             println!("atomic_mpmc thread {} recv {}", i, _i);
    //         } else {
    //             println!("atomic_mpmc rx closed");
    //             break;
    //         }
    //     });
    //     handles.push(handle);
    // }

    // for handle in sender_handles {
    //     handle.join().unwrap();
    // }
    // drop((tx,rx));

    // for handle in handles {
    //     handle.join().unwrap();
    // }
}

// has issues.
pub fn broadcaster() {
    // let rt = tokio::runtime::Runtime::new().unwrap();

    // let mut chan = BroadcastChannel::new();
    // rt.block_on(async move {
    //     chan.send(&5i32).await.unwrap();
    //     println!("chan: {}", chan.next().await.unwrap());

    //     let chan2 = chan.clone();
    //     chan2.send(&6i32).await.unwrap();

    //     println!("chan1: {}", chan.next().await.unwrap());
    //     println!("chan2: {}", chan.next().await.unwrap());
    // });
}

pub fn flume_example() {
    let (tx, rx) = flume::unbounded();

    thread::spawn(move || {
        (0..10).for_each(|i| {
            tx.send(i).unwrap();
        })
    });

    let received: u32 = rx.iter().sum();

    assert_eq!((0..10).sum::<u32>(), received);
}

pub fn async_channel_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (tx, rx) = async_channel::unbounded();

    rt.block_on(async move {
        tokio::spawn(async move {
            tx.send(5).await.unwrap();
        });

        println!("rx: {}", rx.recv().await.unwrap());
    });
}

pub fn async_priority_channel_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (s, r) = async_priority_channel::unbounded();

    rt.block_on(async move {
        tokio::spawn(async move {
            assert_eq!(s.send("Foo", 0).await, Ok(()));
            assert_eq!(s.send("Bar", 2).await, Ok(()));
            assert_eq!(s.send("Baz", 1).await, Ok(()));
        });

        assert_eq!(r.recv().await, Ok(("Bar", 2)));
    });
}
