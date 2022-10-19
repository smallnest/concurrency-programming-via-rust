
use std::time::{Duration, Instant};
use process_lock::ProcessLock;


pub fn process_lock() {
    let lock = ProcessLock::new(String::from(".process_lock"), None);
    let start = Instant::now();
    loop {
        if lock.is_ok() {
            println!("lock success");
            break;
        }
        if start.elapsed() > Duration::from_millis(500) {
            println!("lock timeout");
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    std::thread::sleep(Duration::from_millis(500));

}