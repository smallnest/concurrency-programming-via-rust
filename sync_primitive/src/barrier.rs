use core::time;
use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

use rand::Rng;

pub fn barrier_example() {
    let barrier = Arc::new(Barrier::new(10));
    let mut handles = vec![];

    for _ in 0..10 {
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            let dur = rand::thread_rng().gen_range(100..1000);
            thread::sleep(std::time::Duration::from_millis(dur));

            barrier.wait();
            
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn barrier_recycle_example() {
    let barrier = Arc::new(Barrier::new(10));
    let mut handles = vec![];

    for _ in 0..10 {
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait1");
            let dur = rand::thread_rng().gen_range(100..1000);
            thread::sleep(std::time::Duration::from_millis(dur));

            //step1
            barrier.wait();
            println!("after wait1");
            thread::sleep(time::Duration::from_secs(1));

            //step2
            barrier.wait();
            println!("after wait2");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}