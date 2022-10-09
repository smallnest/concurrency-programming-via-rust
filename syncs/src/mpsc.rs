use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn mpsc_example() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for i in 0..10 {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            let dur = rand::thread_rng().gen_range(100..1000);
            thread::sleep(Duration::from_millis(dur));
            tx.send(i).unwrap();
        }));
    }

    thread::spawn(|| {
        for handle in handles {
            handle.join().unwrap();
        }
        drop(tx)
    });

    for i in rx {
        println!("MPSC: {}", i);
    }
}

pub fn sync_channel_example() {
    let (tx, rx) = mpsc::sync_channel::<i32>(0);
    thread::spawn(move || {
        tx.send(53).unwrap();
    });

    println!("SyncChannel: {}", rx.recv().unwrap());
}
