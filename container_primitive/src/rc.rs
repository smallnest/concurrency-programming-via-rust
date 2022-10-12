
// Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.

use std::rc::Rc;

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