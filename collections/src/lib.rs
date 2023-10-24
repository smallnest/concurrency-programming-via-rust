use cuckoofilter::CuckooFilter;
use dashmap::DashMap;
use std::collections::LinkedList;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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

pub fn common_thread_safe_vec() {
    let vec1 = vec![];
    let vec2 = Arc::new(Mutex::new(vec1));

    let mut handles = vec![];
    for i in 0..10 {
        let vec3 = Arc::clone(&vec2);
        handles.push(std::thread::spawn(move || {
            let mut v = vec3.lock().unwrap();
            v.push(i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("vec: {:?}", vec2.lock().unwrap());
}

pub fn common_thread_safe_linkedlist() {
    let list1: LinkedList<u32> = LinkedList::new();
    let list2 = Arc::new(Mutex::new(list1));

    let mut handles = vec![];
    for i in 0..10 {
        let list3 = Arc::clone(&list2);
        handles.push(std::thread::spawn(move || {
            let mut v = list3.lock().unwrap();
            v.push_back(i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("LinkedList: {:?}", list2.lock().unwrap());
}

pub fn dashmap_example() {
    let map = Arc::new(DashMap::new());
    let mut handles = vec![];

    for i in 0..10 {
        let map = Arc::clone(&map);
        handles.push(std::thread::spawn(move || {
            map.insert(i, i);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("DashMap: {:?}", map);
}

pub fn cuckoofilter_example() {
    let value: &str = "hello world";

    // Create cuckoo filter with default max capacity of 1000000 items
    let mut cf = CuckooFilter::new();

    // Add data to the filter
    cf.add(value).unwrap();

    // Lookup if data is in the filter
    let success = cf.contains(value);
    assert!(success);

    // Test and add to the filter (if data does not exists then add)
    let success = cf.test_and_add(value).unwrap();
    assert!(!success);

    // Remove data from the filter.
    let success = cf.delete(value);
    assert!(success);
}

pub fn evmap_example() {
    let (book_reviews_r, book_reviews_w) = evmap::new();

    // start some writers.
    // since evmap does not support concurrent writes, we need
    // to protect the write handle by a mutex.
    let w = Arc::new(Mutex::new(book_reviews_w));
    let writers: Vec<_> = (0..4)
        .map(|i| {
            let w = w.clone();
            std::thread::spawn(move || {
                let mut w = w.lock().unwrap();
                w.insert(i, true);
                w.refresh();
            })
        })
        .collect();

    // eventually we should see all the writes
    while book_reviews_r.len() < 4 {
       std::thread::yield_now();
    }

    // all the threads should eventually finish writing
    for w in writers.into_iter() {
        assert!(w.join().is_ok());
    }
}
