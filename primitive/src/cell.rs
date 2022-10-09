#![allow(dead_code)]

use std::cell::*;
use std::collections::HashMap;
use std::rc::Rc;

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

pub fn rc_refcell_example() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    // Note that if we had not let the previous borrow of the cache fall out
    // of scope then the subsequent borrow would cause a dynamic thread panic.
    // This is the major hazard of using `RefCell`.
    let total: i32 = shared_map.borrow().values().sum();
    println!("{total}");
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

pub fn myrc_example() {
    let s = example::Rc::new("hello world");
    let s1 = s.clone();

    let v = s1.value();
    println!("myrc value: {}", v);
}

pub mod example {
    use std::cell::Cell;
    use std::marker::PhantomData;
    use std::process::abort;
    use std::ptr::NonNull;

    pub struct Rc<T: ?Sized> {
        ptr: NonNull<RcBox<T>>,
        phantom: PhantomData<RcBox<T>>,
    }

    impl<T> Rc<T> {
        pub fn new(t: T) -> Self {
            let ptr = Box::new(RcBox {
                strong: Cell::new(1),
                refcount: Cell::new(1),
                value: t,
            });
            let ptr = NonNull::new(Box::into_raw(ptr)).unwrap();
            Self {
                ptr: ptr,
                phantom: PhantomData,
            }
        }

        pub fn value(&self) -> &T {
            &self.inner().value
        }
    }


    struct RcBox<T: ?Sized> {
        strong: Cell<usize>,
        refcount: Cell<usize>,
        value: T,
    }

    impl<T: ?Sized> Clone for Rc<T> {
        fn clone(&self) -> Rc<T> {
            self.inc_strong();
            Rc {
                ptr: self.ptr,
                phantom: PhantomData,
            }
        }
    }

    trait RcBoxPtr<T: ?Sized> {
        fn inner(&self) -> &RcBox<T>;

        fn strong(&self) -> usize {
            self.inner().strong.get()
        }

        fn inc_strong(&self) {
            self.inner()
                .strong
                .set(self.strong().checked_add(1).unwrap_or_else(|| abort()));
        }
    }

    impl<T: ?Sized> RcBoxPtr<T> for Rc<T> {
        fn inner(&self) -> &RcBox<T> {
            unsafe { self.ptr.as_ref() }
        }
    }
}
