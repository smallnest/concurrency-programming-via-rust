use std::{sync::{Arc, Mutex}, collections::HashMap};
use std::collections::LinkedList;

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

    println!("vec: {:?}",  vec2.lock().unwrap());
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

    println!("LinkedList: {:?}",  list2.lock().unwrap());
}
