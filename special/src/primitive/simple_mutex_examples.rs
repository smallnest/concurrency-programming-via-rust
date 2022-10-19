use simple_mutex::Mutex;
use std::sync::Arc;
use std::thread;

pub fn simple_mutex_example() {
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let m = m.clone();
        handles.push(thread::spawn(move || {
            *m.lock() += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("m = {:?}", m);
}