use std::thread;
use std::time::Duration;
use std::cell::RefCell;

use thread_priority::*;

pub fn start_one_thread() {
    let count = thread::available_parallelism().unwrap().get();
    println!("available_parallelism: {}", count);

    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}

pub fn start_two_threads() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread1!");
    });

    let handle2 = thread::spawn(|| {
        println!("Hello from a thread2!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn start_thread_with_sleep() {
    let handle1 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        println!("Hello from a thread3!");
    });

    let handle2 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        println!("Hello from a thread4!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn start_thread_with_priority() {
    let handle1 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
        println!("Hello from a thread5!");
    });

    let handle2 = thread::spawn(|| {
        assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        println!("Hello from a thread6!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn thread_builder() {
    let thread1 = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn(|result| {
            println!("Set priority result: {:?}", result);
            assert!(result.is_ok());
        })
        .unwrap();

    let thread2 = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn_careless(|| {
            println!("We don't care about the priority result.");
        })
        .unwrap();

    thread1.join().unwrap();
    thread2.join().unwrap();
}


pub fn start_one_thread_with_move() {
    let x = 100;

    let handle = thread::spawn(move || {
        println!("Hello from a thread, x={}!", x);
    });

    handle.join().unwrap();
}



pub fn start_one_thread_with_thradlocal() {
    thread_local!(static COUNTER: RefCell<u32> = RefCell::new(1));

    COUNTER.with(|c| {
        *c.borrow_mut() = 2;
    });

    let handle1 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 3;
        });

        COUNTER.with(|c| {
            println!("Hello from a thread1, c={}!", *c.borrow());
        });
    });

    let handle2 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 4;
        });

        COUNTER.with(|c| {
            println!("Hello from a thread2, c={}!", *c.borrow());
        });
    });


    handle1.join().unwrap();
    handle2.join().unwrap();

    COUNTER.with(|c| {
        println!("Hello from main, c={}!", *c.borrow());
    });
}


pub fn thread_park() {
    let handle = thread::spawn(|| {
        thread::park();
        println!("Hello from a park thread!");
    });

    thread::sleep(Duration::from_millis(1000));

    handle.thread().unpark();

    handle.join().unwrap();
}

pub fn thread_park_timeout() {
    let handle = thread::spawn(|| {
        thread::park_timeout(Duration::from_millis(1000));
        println!("Hello from a park_timeout thread!");
    });
    handle.join().unwrap();
}

pub fn start_scoped_threads() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    
    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });
    
    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}