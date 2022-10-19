use std::sync::Arc;
use std::thread;
use dashmap::DashMap;

pub fn hashmap_example() {
    let map = Arc::new(DashMap::new());

    let map1 = map.clone();
    let whandle = thread::spawn(move || {
        map1.insert(1, 2);
        map1.insert(2, 3);
    });


    let map2 = map.clone();
    let rhandle = thread::spawn(move || {
        
        loop {
            if let Some(v) = map2.get(&1) {
                println!("get value {} for key 1", *v);
                break;
            } 
        }

        loop {
            if let Some(v) = map2.get(&2) {
                println!("get value {} for key 2", *v);
                break;
            } 
        }
    });

    whandle.join().unwrap();
    rhandle.join().unwrap();
    
}
