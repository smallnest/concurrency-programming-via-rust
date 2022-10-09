use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::thread;

pub fn condvar_example() {
    let condvar = Arc::new(Condvar::new());
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let condvar = condvar.clone();
        let mutex = mutex.clone();
        handles.push(thread::spawn(move || {
            let mut num = mutex.lock().unwrap();
            *num += 1;
            println!("num1: {}", num);
            condvar.notify_one();
        }));
    }

    for _ in 0..10 {
        let condvar = condvar.clone();
        let mutex = mutex.clone();
        handles.push(thread::spawn(move || {
            let mut num = mutex.lock().unwrap();
            while *num == 0 {
                num = condvar.wait(num).unwrap();
            }
            *num += 1;
            println!("num2: {}", num);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Condvar: {}", *mutex.lock().unwrap());
}

pub fn condvar_example2() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_all();
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    // As long as the value inside the `Mutex<bool>` is `false`, we wait.
    // or
    // let _guard = cvar.wait_while(lock.lock().unwrap(), |pending| { *pending }).unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }


    println!("condvar_example2 finished");
}
