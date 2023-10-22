use std::cell::LazyCell;
use std::sync::LazyLock;

pub fn lazy_cell_example() {
    let lazy: LazyCell<i32> = LazyCell::new(|| {
        println!("initializing");
        92
    });
    println!("ready");
    println!("{}", *lazy);
    println!("{}", *lazy);
}

use std::collections::HashMap;
static HASHMAP: LazyLock<HashMap<i32, String>> = LazyLock::new(|| {
    println!("initializing");
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    m
});

pub fn lazy_lock_example() {
    println!("ready");
    std::thread::spawn(|| {
        println!("{:?}", HASHMAP.get(&13));
    }).join().unwrap();
    println!("{:?}", HASHMAP.get(&74));
}