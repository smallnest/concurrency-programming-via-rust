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

pub fn futures_channel_mpsc_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (tx, mut rx) = futures_channel::mpsc::channel(3);

    rt.block_on(async move {
        tokio::spawn(async move {
            for _ in 0..3 {
                let mut tx = tx.clone();
                thread::spawn(move || tx.start_send("ok"));
            }

            drop(tx);
        });

        // Unbounded receiver waiting for all senders to complete.
        while let Ok(msg) = rx.try_next() {
            println!("{:?}", msg);
        }

        println!("futures_channel_mpsc_example completed");
    });
}

pub fn futures_channel_oneshot_example() {
    use futures::channel::oneshot;
    use std::time::Duration;

    let (sender, receiver) = oneshot::channel::<i32>();

    thread::spawn(|| {
        println!("THREAD: sleeping zzz...");
        thread::sleep(Duration::from_millis(1000));
        println!("THREAD: i'm awake! sending.");
        sender.send(3).unwrap();
    });

    println!("MAIN: doing some useful stuff");

    futures::executor::block_on(async {
        println!("MAIN: waiting for msg...");
        println!("MAIN: got: {:?}", receiver.await)
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

pub fn kanal_example() {
    let (tx, rx) = kanal::unbounded();

    thread::spawn(move || {
        (0..10).for_each(|i| {
            tx.send(i).unwrap();
        });

        drop(tx)
    });

    let received: u32 = rx.sum();
    
    println!("received sum: {}", received);
}

pub fn kanal_async_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (tx, rx) = kanal::unbounded_async();

    rt.block_on(async move {
        tokio::spawn(async move {
            tx.send(5).await.unwrap();
        });

        println!("rx: {}", rx.recv().await.unwrap());
    });
}