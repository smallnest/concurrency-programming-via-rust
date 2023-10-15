#![allow(dead_code)]

use std::cell::*;
use std::collections::HashMap;
use std::sync::LazyLock;

// Shareable mutable containers.
// Shareable mutable containers exist to permit mutability in a controlled manner, even in the presence of aliasing.
// Both Cell<T> and RefCell<T> allow doing this in a single-threaded way. However, neither Cell<T> nor RefCell<T> are thread safe (they do not implement Sync).
// If you need to do aliasing and mutation between multiple threads it is possible to use Mutex<T>, RwLock<T> or atomic types.
//
// Cell<T> implements interior mutability by moving values in and out of the Cell<T>.
// To use references instead of values, one must use the RefCell<T> type, acquiring a write lock before mutating.

pub fn cell_example() {
    struct SomeStruct {
        regular_field: u8,
        special_field: Cell<u8>,
    }

    // not mutable
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };

    let _ = my_struct.regular_field;
    // my_struct.regular_field = 100;
    my_struct.special_field.set(100);

    my_struct.special_field.update(|v| v + 1);
}

pub fn refcell_example() {
    #[allow(dead_code)]
    struct SomeStruct {
        regular_field: u8,
        special_field: RefCell<u8>,
    }

    // not mutable
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: RefCell::new(1),
    };

    // my_struct.regular_field = 100;
    let mut special_field = (&my_struct.special_field).borrow_mut();
    *special_field = 100;
    drop(special_field);

    println!("special_field = {}", my_struct.special_field.borrow());

    (&my_struct).special_field.replace(200);
    println!("special_field = {}", my_struct.special_field.borrow());
}



pub fn once_cell_example() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());

    let value: &String = cell.get_or_init(|| "Hello, World!".to_string());
    assert_eq!(value, "Hello, World!");
    assert!(cell.get().is_some());
}

pub fn lazy_cell_example() {
    let lazy: LazyCell<i32> = LazyCell::new(|| {
        println!("initializing");
        92
    });
    println!("one_cell ready");
    println!("{}", *lazy);
    println!("{}", *lazy);
}

static HASHMAP: LazyLock<HashMap<i32, String>> = LazyLock::new(|| {
    println!("initializing");
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    m
});

pub fn lazy_lock() {
    println!("ready");
    std::thread::spawn(|| {
        println!("{:?}", HASHMAP.get(&13));
    }).join().unwrap();
    println!("{:?}", HASHMAP.get(&74));
}