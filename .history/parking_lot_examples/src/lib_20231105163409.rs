use std::sync::{mpsc::channel, Arc};
use std::thread;

use parking_lot::RwLock;
use parking_lot::{FairMutex, Mutex, Once, ReentrantMutex};

pub fn mutex_example() {
    const N: usize = 10;

    let data = Arc::new(Mutex::new(0));
    let data2 = &data.clone();

    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (Arc::clone(&data), tx.clone());
        thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            let mut data = data.lock();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
            // the lock is unlocked here when `data` goes out of scope.
        });
    }

    rx.recv().unwrap();

    println!("mutex_example: {}", data2.lock());
}

pub fn mutex_example2() {
    const N: usize = 10;

    let mutex = Arc::new(Mutex::new(()));

    let handles: Vec<_> = (0..N)
        .map(|i| {
            let mutex = Arc::clone(&mutex);
            thread::spawn(move || {
                let _lock = mutex.lock();
                println!("thread {} done", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("mutex_example2: done");
}

pub fn mutex_example3() {
    const N: usize = 10;

    let mutex = Arc::new(Mutex::new(()));

    let handles: Vec<_> = (0..N)
        .map(|i| {
            let mutex = Arc::clone(&mutex);
            thread::spawn(move || match mutex.try_lock() {
                Some(_guard) => println!("thread {} got the lock", i),
                None => println!("thread {} did not get the lock", i),
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("mutex_example3: done");
}

pub fn mutex_example4() {
    use parking_lot::Mutex;
    use std::mem;

    let mutex = Mutex::new(1);

    // 使用mem::forget持有锁直到结束
    let _guard = mem::forget(mutex.lock());

    // 一些访问受mutex保护的数据的代码

    // 在结束前解锁mutex
    unsafe {
        mutex.force_unlock();
    }

    println!("mutex_example4: done");
}

pub fn fairmutex_example() {
    const N: usize = 10;

    let data = Arc::new(FairMutex::new(0));

    let (tx, rx) = channel();
    for _ in 0..10 {
        let (data, tx) = (Arc::clone(&data), tx.clone());
        thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            let mut data = data.lock();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
            // the lock is unlocked here when `data` goes out of scope.
        });
    }

    rx.recv().unwrap();

    println!("fairmutex_example: done");
}

pub fn rwmutex_example() {
    const N: usize = 10;

    let lock = Arc::new(RwLock::new(5));

    let handles: Vec<_> = (0..N)
        .map(|i| {
            let lock = Arc::clone(&lock);
            thread::spawn(move || {
                if i % 2 == 0 {
                    let mut num = lock.write();
                    *num += 1;
                } else {
                    let num = lock.read();
                    println!("thread {} read {}", i, num);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("rwmutex_example: {}", lock.read());
}
pub fn reentrantmutex_example() {
    let lock = ReentrantMutex::new(());

    reentrant(&lock, 10);

    println!("reentrantMutex_example: done");
}

fn reentrant(lock: &ReentrantMutex<()>, i: usize) {
    if i == 0 {
        return;
    }

    let _lock = lock.lock();
    reentrant(lock, i - 1);
}

pub fn once_example() {
    static mut VAL: usize = 0;
    static INIT: Once = Once::new();
    fn get_cached_val() -> usize {
        unsafe {
            INIT.call_once(|| {
                println!("initializing once");
                thread::sleep(std::time::Duration::from_secs(1));
                VAL = 100;
            });
            VAL
        }
    }

    let handle = thread::spawn(|| {
        println!("thread 1 get_cached_val: {}", get_cached_val());
    });

    println!("get_cached_val: {}", get_cached_val());

    handle.join().unwrap();
}

pub fn condvar_example() {
    use std::sync::Condvar;
    use std::sync::Mutex;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap(); // block until notified
    }
    println!("condvar_example: done");
}
