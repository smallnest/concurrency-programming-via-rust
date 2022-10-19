use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossbeam::thread as crossbeam_thread;
use go_spawn::{go, join};
use parking::Parker;
use rayon;
use send_wrapper::SendWrapper;
use thread_amount::thread_amount;
use thread_control::*;
use thread_priority::*;

#[cfg(not(target_os = "macos"))]
use affinity::*;

pub fn start_one_thread() {
    let count = thread::available_parallelism().unwrap().get();

    println!("available_parallelism: {}", count);

    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}

pub fn start_one_thread_result() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
        200
    });

    match handle.join() {
        Ok(v) => println!("thread result: {}", v),
        Err(e) => println!("error: {:?}", e),
    }
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

pub fn start_n_threads() {
    const N: isize = 10;

    let handles: Vec<_> = (0..N)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from a thread{}!", i);
            })
        })
        .collect();

    // handles.into_iter().for_each(|h| h.join().unwrap());

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn current_thread() {
    let current_thread = thread::current();
    println!(
        "current thread: {:?},{:?}",
        current_thread.id(),
        current_thread.name()
    );

    let builder = thread::Builder::new()
        .name("foo".into()) // set thread name
        .stack_size(32 * 1024); // set stack size

    let handler = builder
        .spawn(|| {
            let current_thread = thread::current();
            println!(
                "child thread: {:?},{:?}",
                current_thread.id(),
                current_thread.name()
            );
        })
        .unwrap();

    handler.join().unwrap();
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

pub fn start_thread_with_yield_now() {
    let handle1 = thread::spawn(|| {
        thread::yield_now();
        println!("yield_now!");
    });

    let handle2 = thread::spawn(|| {
        thread::yield_now();
        println!("yield_now in another thread!");
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
        println!("Hello from a thread with move, x={}!", x);
    });

    handle.join().unwrap();

    let handle = thread::spawn(move || {
        println!("Hello from a thread with move again, x={}!", x);
    });
    handle.join().unwrap();

    let handle = thread::spawn(|| {
        println!("Hello from a thread without move");
    });
    handle.join().unwrap();
}

// pub fn start_one_thread_with_move2() {
//     let x = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Hello from a thread with move, x={:?}!", x);
//     });

//     handle.join().unwrap();

//     let handle = thread::spawn(move|| {
//         println!("Hello from a thread with move again, x={:?}!", x);
//     });
//     handle.join().unwrap();

//     let handle = thread::spawn(|| {
//         println!("Hello from a thread without move");
//     });
//     handle.join().unwrap();

// }

pub fn start_threads_with_threadlocal() {
    thread_local!(static COUNTER: RefCell<u32> = RefCell::new(1));

    COUNTER.with(|c| {
        *c.borrow_mut() = 2;
    });

    let handle1 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 3;
        });

        COUNTER.with(|c| {
            println!("Hello from a thread7, c={}!", *c.borrow());
        });
    });

    let handle2 = thread::spawn(move || {
        COUNTER.with(|c| {
            *c.borrow_mut() = 4;
        });

        COUNTER.with(|c| {
            println!("Hello from a thread8, c={}!", *c.borrow());
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

pub fn thread_park2() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });

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

// pub fn wrong_start_threads_without_scoped() {
//     let mut a = vec![1, 2, 3];
//     let mut x = 0;

//     thread::spawn(move || {
//         println!("hello from the first scoped thread");
//         dbg!(&a);
//     });
//     thread::spawn(move || {
//         println!("hello from the second scoped thread");
//         x += a[0] + a[2];
//     });
//     println!("hello from the main thread");

//     // After the scope, we can modify and access our variables again:
//     a.push(4);
//     assert_eq!(x, a.len());
// }

pub fn start_scoped_threads() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}

pub fn crossbeam_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    crossbeam_thread::scope(|s| {
        s.spawn(|_| {
            println!("hello from the first crossbeam scoped thread");
            dbg!(&a);
        });
        s.spawn(|_| {
            println!("hello from the second crossbeam scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    })
    .unwrap();

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}

pub fn rayon_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    rayon::scope(|s| {
        s.spawn(|_| {
            println!("hello from the first rayon scoped thread");
            dbg!(&a);
        });
        s.spawn(|_| {
            println!("hello from the second rayon scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());
}

// pub fn wrong_send() {
//     let counter = Rc::new(42);

//     let (sender, receiver) = channel();

//     let _t = thread::spawn(move || {
//         sender.send(counter).unwrap();
//     });

//     let value = receiver.recv().unwrap();

//     println!("received from the main thread: {}", value);
// }

pub fn send_wrapper() {
    let wrapped_value = SendWrapper::new(Rc::new(42));

    let (sender, receiver) = channel();

    let _t = thread::spawn(move || {
        sender.send(wrapped_value).unwrap();
    });

    let wrapped_value = receiver.recv().unwrap();

    let value = wrapped_value.deref();
    println!("received from the main thread: {}", value);
}

pub fn print_thread_amount() {
    let mut handles = vec![];

    for _ in 1..=10 {
        let amount = thread_amount();

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            if !amount.is_none() {
                println!("thread amount: {}", amount.unwrap());
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn control_thread() {
    let (flag, control) = make_pair();
    let handle = thread::spawn(move || {
        while flag.alive() {
            thread::sleep(Duration::from_millis(100));
            println!("I'm alive!");
        }
    });

    thread::sleep(Duration::from_millis(100));
    assert_eq!(control.is_done(), false);
    control.stop(); // Also you can `control.interrupt()` it
    handle.join().unwrap();

    assert_eq!(control.is_interrupted(), false);
    assert_eq!(control.is_done(), true);

    println!("This thread is stopped")
}

#[cfg(not(target_os = "macos"))]
pub fn use_affinity() {
    // Select every second core
    let cores: Vec<usize> = (0..get_core_num()).step_by(2).collect();
    println!("Binding thread to cores : {:?}", &cores);

    affinity::set_thread_affinity(&cores).unwrap();
    println!(
        "Current thread affinity : {:?}",
        affinity::get_thread_affinity().unwrap()
    );
}

pub fn go_thread() {
    let counter = Arc::new(AtomicI64::new(0));
    let counter_cloned = counter.clone();

    // Spawn a thread that captures values by move.
    go! {
        for _ in 0..100 {
            counter_cloned.fetch_add(1, Ordering::SeqCst);
        }
    }

    // Join the most recent thread spawned by `go_spawn` that has not yet been joined.
    assert!(join!().is_ok());
    assert_eq!(counter.load(Ordering::SeqCst), 100);
}

pub fn park_thread() {
    let p = Parker::new();
    let u = p.unparker();

    // Notify the parker.
    u.unpark();

    // Wakes up immediately because the parker is notified.
    p.park();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        u.unpark();
    });

    // Wakes up when `u.unpark()` notifies and then goes back into unnotified state.
    p.park();

    println!("park_unpark")
}

pub fn info() {
    let count = thread::available_parallelism().unwrap().get();
    println!("available_parallelism: {}", count);

    if let Some(count) = num_threads::num_threads() {
        println!("num_threads: {}", count);
    } else {
        println!("num_threads: not supported");
    }

    let count = thread_amount::thread_amount();
    if !count.is_none() {
        println!("thread_amount: {}", count.unwrap());
    }

    let count = num_cpus::get();
    println!("num_cpus: {}", count);
}
