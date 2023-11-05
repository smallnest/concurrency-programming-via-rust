use std::ops::Add;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use chrono::{self, Local};
use timer;

pub fn timer_schedule_with_delay() {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
        // This closure is executed on the scheduler thread,
        // so we want to move it away asap.

        let _ignored = tx.send(()); // Avoid unwrapping here.
    });

    rx.recv().unwrap();
    println!("This code has been executed after 3 seconds");
}

pub fn timer_schedule_with_date() {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    let _guard =
        timer.schedule_with_date(Local::now().add(chrono::Duration::seconds(1)), move || {
            // This closure is executed on the scheduler thread,
            // so we want to move it away asap.

            let _ignored = tx.send(()); // Avoid unwrapping here.
        });

    rx.recv().unwrap();
    println!("This code has been executed after 1 seconds");
}

pub fn timer_repeat() {
    let timer = timer::Timer::new();
    // Number of times the callback has been called.
    let count = Arc::new(Mutex::new(0));

    // Start repeating. Each callback increases `count`.
    let guard = {
        let count = count.clone();
        timer.schedule_repeating(chrono::Duration::milliseconds(5), move || {
            *count.lock().unwrap() += 1;
        })
    };

    // Sleep one second. The callback should be called ~200 times.
    thread::sleep(std::time::Duration::new(1, 0));
    let count_result = *count.lock().unwrap();
    assert!(
        190 <= count_result && count_result <= 210,
        "The timer was called {} times",
        count_result
    );

    // Now drop the guard. This should stop the timer.
    drop(guard);
    thread::sleep(std::time::Duration::new(0, 100));

    // Let's check that the count stops increasing.
    let count_start = *count.lock().unwrap();
    thread::sleep(std::time::Duration::new(1, 0));
    let count_stop = *count.lock().unwrap();
    assert_eq!(count_start, count_stop);
}

pub fn safina_timer_example() {
    smol::block_on(async {
        safina_timer::start_timer_thread();
        let duration = std::time::Duration::from_secs(1);
        safina_timer::sleep_for(duration).await;
    });

    smol::block_on(async {
        safina_timer::start_timer_thread();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(1);
        safina_timer::sleep_until(deadline).await;
    });
}

pub fn futures_timer_example() {
    use futures_timer::Delay;
    use std::time::Duration;

    smol::block_on(async {
        for _ in 0..5 {
            Delay::new(Duration::from_secs(1)).await;
            println!("重复定时任务触发！");
        }
    });
}

pub fn async_io_timer_example() {
    use async_io::Timer;
    use std::time::Duration;

    let timer = Timer::after(Duration::from_secs(1));

    smol::block_on(async {
        timer.await;
        println!("一秒过去了");
    });
}
