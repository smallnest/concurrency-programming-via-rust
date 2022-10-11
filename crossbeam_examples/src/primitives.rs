use crossbeam::thread;
use std::sync::Arc;
use std::time::*;

use crossbeam::channel::*;
use crossbeam::sync::Parker;
use crossbeam::sync::ShardedLock;
use crossbeam::sync::WaitGroup;

pub fn unbounded_channel_example() {
    let (sender, receiver) = unbounded();

    let sender1 = sender.clone();

    thread::scope(|s| {
        s.spawn(|_| {
            sender.send(1).unwrap();
            sender.send(2).unwrap();
            sender1.send(3).unwrap();
        });

        assert_eq!(receiver.recv(), Ok(1));
        assert_eq!(receiver.recv(), Ok(2));
        assert_eq!(receiver.recv(), Ok(3));
    })
    .unwrap();
}

pub fn bounded_channel_example() {
    let (sender, receiver) = bounded(2);

    thread::scope(|s| {
        s.spawn(|_| {
            sender.send(1).unwrap();
            sender.send(2).unwrap();
            sender.send(3).unwrap();
        });

        assert_eq!(receiver.recv(), Ok(1));
        assert_eq!(receiver.recv(), Ok(2));
        assert_eq!(receiver.recv(), Ok(3));
    })
    .unwrap();
}

pub fn channel_iter_example() {
    let (sender, receiver) = unbounded();

    thread::scope(|s| {
        s.spawn(|_| {
            sender.send(1).unwrap();
            sender.send(2).unwrap();
            sender.send(3).unwrap();
            drop(sender); // Disconnect the channel.
        });

        let v: Vec<i32> = receiver.iter().collect();
        println!("channel_iter_example: {:?}", v);
    })
    .unwrap();
}

pub fn channel_select_example() {
    let (s1, r1) = unbounded();
    let (s2, r2) = unbounded();

    std::thread::spawn(move || s1.send(10).unwrap());
    std::thread::spawn(move || s2.send(20).unwrap());

    // At most one of these two receive operations will be executed.
    select! {
        recv(r1) -> msg => println!("r1 received {}", msg.unwrap()),
        recv(r2) -> msg => println!("r2 received {}", msg.unwrap()),
        default(Duration::from_secs(1)) => println!("timed out"),
    }
}

pub fn channel_extra_example() {
    let (s1, r1) = unbounded();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(2));
        s1.send(10).unwrap()
    });

    let start = Instant::now();
    let ticker = tick(Duration::from_millis(50));
    let timeout = after(Duration::from_secs(1));

    loop {
        select! {
            recv(r1) -> msg => {println!("r1 received {}", msg.unwrap());break},
            recv(ticker) -> _ => println!("elapsed: {:?}", start.elapsed()),
            recv(timeout) -> _ => {println!("elapsed: {:?}, timeout", start.elapsed());break},
        }
    }
}

pub fn sharded_lock_example() {
    let lock = Arc::new(ShardedLock::new(0));

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let lock = lock.clone();
            std::thread::spawn(move || {
                if i % 2 == 0 {
                    let mut num = lock.write().unwrap();
                    *num += 1;
                } else {
                    let num = lock.read().unwrap();
                    println!("thread {} read {}", i, num);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn waitgroup_example() {
    let wg = WaitGroup::new();

    for i in 0..4 {
        let wg = wg.clone();
        std::thread::spawn(move || {
            println!("waitgroup_example thread: {}", i);
            drop(wg); // wg.wait();
        });
    }
    wg.wait();

    println!("waitgroup_example: done");
}

pub fn parker_example() {
    let p = Parker::new();
    let u = p.unparker().clone();

    // Make the token available.
    u.unpark();
    // Wakes up immediately and consumes the token.
    p.park();

    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(500));
        u.unpark();
    });

    // Wakes up when `u.unpark()` provides the token.
    println!("parking...");
    p.park();
    println!("unparked");
}
