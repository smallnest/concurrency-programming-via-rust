
use arc_swap::ArcSwap;
use std::sync::Arc;
use crossbeam_utils::thread;

pub fn arc_swap_example() {
    let value = ArcSwap::from(Arc::new(5));
    thread::scope(|scope| {
        scope.spawn(|_| {
            let new_value = Arc::new(4);
            value.store(new_value);
        });
        for _ in 0..10 {
            scope.spawn(|_| {
                loop {
                    let v = value.load();
                    println!("value is {}", v);
                    return;
                }
            }); 
        }   
    }).unwrap()
    
}