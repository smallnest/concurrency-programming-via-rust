use crossbeam::atomic::{AtomicCell,AtomicConsume};
use std::thread;

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

pub fn atomic_consume_example() {
    use crossbeam::atomic::AtomicConsume;

    let data = vec![1, 2, 3, 4, 5];
    let shared_data = AtomicConsume::new(data);

    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let shared_data = shared_data.clone();

        join_handles.push(thread::spawn(move || {
            let x = shared_data.load_consume();
            if let Some(v) = x {
                println!("Consumed: {}", v);
            }
        }));
    }

    for handle in join_handles {
        handle.join().unwrap();
    }
}
