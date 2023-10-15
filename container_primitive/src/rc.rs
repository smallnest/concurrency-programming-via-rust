
// Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell,RefMut};

pub fn rc_example() {
    let rc = Rc::new(1);

    let rc2 = rc.clone();
    let rc3 = Rc::clone(&rc);
    println!("rc2: {}, rc3:{}", rc2, rc3);

    let my_weak = Rc::downgrade(&rc);
    drop(rc);
    drop(rc2);
    drop(rc3);

    println!("my_weak: {}", my_weak.upgrade().is_none());

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
