use concurrent_queue::ConcurrentQueue;
use std::sync::Arc;
use std::thread;

use triple_buffer::triple_buffer;

pub fn concurrent_queue_example() {
    let q = Arc::new(ConcurrentQueue::unbounded());

    let q1 = q.clone();
    let whandle = thread::spawn(move || {
        for i in 0..10 {
            q1.push(i).unwrap();
        }
    });

    let q2 = q.clone();
    let rhandle = thread::spawn(move || loop {
        if let Ok(v) = q2.pop() {
            println!("get value {}", v);
        } else {
            println!("queue closed");
            break;
        }
    });

    whandle.join().unwrap();
    rhandle.join().unwrap();
}

pub fn triple_buffer_example() {
    let (mut buf_input, mut buf_output) = triple_buffer(&0);

    // The producer thread can move a value into the buffer at any time
    let producer = std::thread::spawn(move || buf_input.write(42));

    // The consumer thread can read the latest value at any time
    let consumer = std::thread::spawn(move || {
        let latest = buf_output.read();
        assert!(*latest == 42 || *latest == 0);
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
