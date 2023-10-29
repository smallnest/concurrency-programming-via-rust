pub mod others;
pub use others::*;

use std::sync::mpsc;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Duration;

pub fn mpsc_example1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn mpsc_example2() {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 0..10 {
        let j = rx.recv().unwrap();
        println!("Got: {}", j);
    }
}

pub fn mpsc_example3() {
    let (tx, rx) = sync_channel::<i32>(0);
    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        tx.send(53).unwrap();
    });
    rx.recv().unwrap();
}

pub fn mpsc_example4() {
    let (tx, rx) = sync_channel(3);

    for _ in 0..3 {
        let tx = tx.clone();
        // cloned tx dropped within thread
        thread::spawn(move || tx.send("ok").unwrap());
    }

    drop(tx);

    // Unbounded receiver waiting for all senders to complete.
    while let Ok(msg) = rx.recv() {
        println!("{msg}");
    }

    println!("mpsc_example4 completed");
}

pub fn mpsc_drop_example() {
        // 创建一个有边界的多生产者、单消费者的通道
        let (sender, receiver) = mpsc::channel::<i32>(); // 指定通道中传递的数据类型为 i32

        // 启动三个生产者线程
        for i in 0..3 {
            let tx = sender.clone(); // 克隆发送端，每个线程都拥有独立的发送端
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(1)); // 等待所有线程启动完毕
                tx.send(i).expect("Failed to send message");
            });
        }
        
        
        // 丢弃发送端，不影响clone
        drop(sender); 
      
    
        // 主线程作为消费者，接收来自生产者线程的消息
        for received_message in receiver {
            println!("Received message: {}", received_message);
        }
}