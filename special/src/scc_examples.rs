use scc::*;
use std::collections::hash_map::RandomState;

pub fn scc_hashmap() {
    let hashmap: HashMap<usize, usize, RandomState> = HashMap::with_capacity(1000);
    assert_eq!(hashmap.capacity(), 1024);

    let ticket = hashmap.reserve(10000);
    assert!(ticket.is_some());
    assert_eq!(hashmap.capacity(), 16384);
    for i in 0..16 {
        assert!(hashmap.insert(i, i).is_ok());
    }
    drop(ticket);

    assert_eq!(hashmap.capacity(), 1024);
}

pub fn scc_hashindex() {
    let hashindex: HashIndex<u64, u32> = HashIndex::default();

    assert!(!hashindex.remove(&1));
    assert!(hashindex.insert(1, 0).is_ok());
    assert!(hashindex.remove(&1));
}

pub fn scc_treeindex() {
    let treeindex: TreeIndex<u64, u32> = TreeIndex::new();

    assert!(treeindex.insert(1, 10).is_ok());
    assert_eq!(treeindex.insert(1, 11).err().unwrap(), (1, 11));
    assert_eq!(treeindex.read(&1, |_k, v| *v).unwrap(), 10);
}

pub fn scc_hashset() {
    let hashset: HashSet<usize, RandomState> = HashSet::with_capacity(1000);
    assert_eq!(hashset.capacity(), 1024);

    let ticket = hashset.reserve(10000);
    assert!(ticket.is_some());
    assert_eq!(hashset.capacity(), 16384);
    for i in 0..16 {
        assert!(hashset.insert(i).is_ok());
    }
    drop(ticket);

    assert_eq!(hashset.capacity(), 1024);
}

pub fn scc_queue() {
    let queue: Queue<usize> = Queue::default();

    queue.push(37);
    queue.push(3);
    queue.push(1);

    assert_eq!(queue.pop().map(|e| **e), Some(37));
    assert_eq!(queue.pop().map(|e| **e), Some(3));
    assert_eq!(queue.pop().map(|e| **e), Some(1));
    assert!(queue.pop().is_none());
}
