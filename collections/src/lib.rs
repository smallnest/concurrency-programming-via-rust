use std::{sync::{Arc, Mutex}, collections::HashMap};


pub fn common_thread_safe_collections() {
    let map: HashMap<i32, i32> = HashMap::new();
    let m = Arc::new(Mutex::new(map));

    let mut handles = vec![];
    for i in 0..10 {
        let m = Arc::clone(&m);
        handles.push(std::thread::spawn(move || {
            let mut map = m.lock().unwrap();
            map.insert(i, i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("HashMap: {:?}", *m.lock().unwrap());
}