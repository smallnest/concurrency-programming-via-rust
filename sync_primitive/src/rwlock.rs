
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

pub fn rwlock_example() {
    let rwlock = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let rwlock = rwlock.clone();
        handles.push(thread::spawn(move || {
            let num = rwlock.read().unwrap();
            println!("num1: {}", num);
        }));
    }

    for _ in 0..10 {
        let rwlock = rwlock.clone();
        handles.push(thread::spawn(move || {
            let mut num = rwlock.write().unwrap();
            *num += 1;
        }));
    }
    for _ in 0..10 {
        let rwlock = rwlock.clone();
        handles.push(thread::spawn(move || {
            let num = rwlock.read().unwrap();
            println!("num2: {}", num);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("RwLock: {}", *rwlock.read().unwrap());
}   
pub fn read_after_write() {
    // 创建一个可共享的可变整数，使用RwLock包装
    let counter = Arc::new(RwLock::new(0));

    // 创建一个线程持有读锁
    let read_handle = {
        let counter = counter.clone();
        thread::spawn(move || {
            // 获取读锁
            let num = counter.read().unwrap();
            println!("Reader#1: {}", *num);

            // 休眠模拟读取操作
            thread::sleep(std::time::Duration::from_secs(10));
        })
    };

    // 创建一个线程请求写锁
    let write_handle = {
        let counter = counter.clone();
        thread::spawn(move || {
            // 休眠一小段时间，确保读锁已经被获取
            thread::sleep(std::time::Duration::from_secs(1));

            // 尝试获取写锁
            let mut num = counter.write().unwrap();
            *num += 1;
            println!("Writer : Incremented counter to {}",  *num);
        })
    };

    // 创建一个线程请求读锁
    let read_handle_2 = {
        let counter = counter.clone();
        thread::spawn(move || {
            // 休眠一小段时间，确保写锁已经被获取
            thread::sleep(std::time::Duration::from_secs(2));

            // 尝试获取读锁
            let num = counter.read().unwrap();
            println!("Reader#2: {}", *num);
        })
    };

    // 等待读取线程和写入线程完成
    read_handle.join().unwrap();
    write_handle.join().unwrap();
    read_handle_2.join().unwrap();
}