use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use fast_threadpool::ThreadPoolConfig;
use futures_lite::*;
use rayon;
use rusty_pool;

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of `pool`
    return a + b;
}

pub fn rayon_threadpool() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let n = pool.install(|| fib(20));
    println!("{}", n);
}

pub fn threadpool_example() {
    let n_workers = 4;
    let n_jobs = 8;
    let pool = threadpool::ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1)
                .expect("channel will be there waiting for the pool");
        });
    }

    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
}

pub fn rusty_pool_example() {
    let pool = rusty_pool::ThreadPool::default();

    for _ in 1..10 {
        pool.execute(|| {
            println!("Hello from a rusty_pool!");
        });
    }

    pool.join();
}

pub fn fast_threadpool_example() -> Result<(), fast_threadpool::ThreadPoolDisconnected> {
    let threadpool =
        fast_threadpool::ThreadPool::start(ThreadPoolConfig::default(), ()).into_sync_handler();

    assert_eq!(4, threadpool.execute(|_| { 2 + 2 })?);

    Ok(())
}

pub fn scoped_threadpool() {
    let mut pool = scoped_threadpool::Pool::new(4);

    let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // Use the threads as scoped threads that can
    // reference anything outside this closure
    pool.scoped(|s| {
        // Create references to each element in the vector ...
        for e in &mut vec {
            // ... and add 1 to it in a seperate thread
            s.execute(move || {
                *e += 1;
            });
        }
    });

    assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

pub fn scheduled_thread_pool() {
    let (sender, receiver) = channel();

    let pool = scheduled_thread_pool::ScheduledThreadPool::new(4);
    let handle = pool.execute_after(Duration::from_millis(1000), move || {
        println!("Hello from a scheduled thread!");
        sender.send("done").unwrap();
    });

    let _ = handle;
    receiver.recv().unwrap();
}

// workerpool-rs
pub fn workerpool_rs_example() {
    use workerpool_rs::pool::WorkerPool;

    let n_workers = 4;
    let n_jobs = 8;
    let pool = WorkerPool::new(n_workers);

    let (tx, rx) = channel();
    let atx = Arc::new(Mutex::new(tx));
    for _ in 0..n_jobs {
        let atx = atx.clone();
        pool.execute(move || {
            let tx = atx.lock().unwrap();
            tx.send(1)
                .expect("channel will be there waiting for the pool");
        });
    }

    // assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
    println!("{}", rx.iter().take(n_jobs).fold(0, |a, b| a + b))
}

fn test(msg: usize) {
    println!("key: {}\tvalue: {}", msg, fib(msg));
}

// poolite
pub fn poolite_example() {
    let pool = poolite::Pool::new().unwrap();
    for i in 0..10 {
        pool.push(move || test(i));
    }

    pool.join(); //wait for the pool
}

pub fn poolite_example2() {
    let pool = poolite::Pool::new().unwrap();
    let mut array = (0..10usize).into_iter().map(|i| (i, 0)).collect::<Vec<_>>();

    // scoped method will waiting scoped's task running finish.
    pool.scoped(|scope| {
        for i in array.iter_mut() {
            // have to move
            scope.push(move || i.1 = i.0 * i.0);
        }
    });

    for (i, j) in array {
        println!("key: {}\tvalue: {}", i, j);
    }
}

pub fn executor_service_example() {
    use executor_service::Executors;

    let mut executor_service =
        Executors::new_fixed_thread_pool(10).expect("Failed to create the thread pool");

    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..10 {
        let counter = counter.clone();
        executor_service.execute(move || {
            thread::sleep(Duration::from_millis(100));
            counter.fetch_add(1, Ordering::SeqCst);
        });
    }

    thread::sleep(Duration::from_millis(1000));

    assert_eq!(counter.load(Ordering::SeqCst), 10);

    let mut executor_service =
        Executors::new_fixed_thread_pool(2).expect("Failed to create the thread pool");

    let some_param = "Mr White";
    let res = executor_service
        .submit_sync(move || {
            sleep(Duration::from_secs(5));
            println!("Hello {:}", some_param);
            println!("Long computation finished");
            2
        })
        .expect("Failed to submit function");

    println!("Result: {:#?}", res);
    assert_eq!(res, 2);
}

pub fn threadpool_executor_example() {
    let pool = threadpool_executor::ThreadPool::new(1);
    let mut expectation = pool.execute(|| "hello, thread pool!").unwrap();
    assert_eq!(expectation.get_result().unwrap(), "hello, thread pool!");

    let pool = threadpool_executor::threadpool::Builder::new()
        .core_pool_size(1)
        .maximum_pool_size(3)
        .keep_alive_time(std::time::Duration::from_secs(300))
        .exeed_limit_policy(threadpool_executor::threadpool::ExceedLimitPolicy::Wait)
        .build();

    pool.execute(|| {
        std::thread::sleep(std::time::Duration::from_secs(3));
    })
    .unwrap();
    let mut exp = pool.execute(|| {}).unwrap();
    exp.cancel();
}

pub fn executors_example() {
    use executors::crossbeam_workstealing_pool;
    use executors::*;
    use std::sync::mpsc::channel;

    let n_workers = 4;
    let n_jobs = 8;
    let pool = crossbeam_workstealing_pool::small_pool(n_workers);

    let (tx, rx) = channel();
    for _ in 0..n_jobs {
        let tx = tx.clone();
        pool.execute(move || {
            tx.send(1)
                .expect("channel will be there waiting for the pool");
        });
    }

    assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
}

// slave-pool
