use crossbeam::atomic::AtomicCell;

pub fn atomic_cell_example() {
    let a = AtomicCell::new(0i32);

    a.store(1);
    assert_eq!(a.load(), 1);

    assert_eq!(a.compare_exchange(1, 2), Ok(1));
    assert_eq!(a.fetch_add(1), 2);
    assert_eq!(a.load(), 3);
    assert_eq!(a.swap(100), 3);
    assert_eq!(a.load(), 100);
    assert_eq!(a.into_inner(), 100);

    let a = AtomicCell::new(100i32);
    let v = a.take();
    assert_eq!(v, 100);
    assert_eq!(a.load(), 0);
}
