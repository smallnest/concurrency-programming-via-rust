use concurrent_queue::ConcurrentQueue;
use std::thread;
use std::sync::Arc;

pub fn concurrent_queue_example() {
    let q = Arc::new(ConcurrentQueue::unbounded());

    let q1 = q.clone();
    let whandle = thread::spawn(move || {
        for i in 0..10 {
            q1.push(i).unwrap();
        }
    });

    let q2 = q.clone();
    let rhandle = thread::spawn(move || {
        loop {
            if let Ok(v) = q2.pop() {
                println!("get value {}", v);
            } else {
                println!("queue closed");
                break;
            }
        }
    });

    whandle.join().unwrap();
    rhandle.join().unwrap();
}
