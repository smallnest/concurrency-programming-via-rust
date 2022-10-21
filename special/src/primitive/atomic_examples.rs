use atomicbox::AtomicBox;
use atomig::Atomic;
use portable_atomic::*;
use std::sync::atomic::Ordering;
use std::sync::atomic::Ordering::Relaxed;

pub fn portable_atomic_i128() {
    let mut some_var = AtomicI128::new(10);
    assert_eq!(*some_var.get_mut(), 10);
    *some_var.get_mut() = 5;
    assert_eq!(some_var.load(Ordering::SeqCst), 5);

    assert_eq!(some_var.load(Ordering::Relaxed), 5);
}

pub fn portable_atomic_u128() {
    let mut some_var = AtomicU128::new(10);
    assert_eq!(*some_var.get_mut(), 10);
    *some_var.get_mut() = 5;
    assert_eq!(some_var.load(Ordering::SeqCst), 5);

    assert_eq!(some_var.load(Ordering::Relaxed), 5);
}

pub fn portable_atomic_f32() {
    let mut some_var = AtomicF32::new(10.0);
    assert_eq!(*some_var.get_mut(), 10.0);
    *some_var.get_mut() = 5.0;
    assert_eq!(some_var.load(Ordering::SeqCst), 5.0);

    assert_eq!(some_var.load(Ordering::Relaxed), 5.0);
}

pub fn portable_atomic_f64() {
    let mut some_var = AtomicF64::new(10.0f64);
    assert_eq!(*some_var.get_mut(), 10.0);
    *some_var.get_mut() = 5.0;
    assert_eq!(some_var.load(Ordering::SeqCst), 5.0);

    assert_eq!(some_var.load(Ordering::Relaxed), 5.0);
}

pub fn atomic_float_example() {
    let some_var = atomic_float::AtomicF32::new(800.0f32);
    some_var.fetch_add(30.0, Relaxed);
    some_var.fetch_sub(-55.0, Relaxed);
    some_var.fetch_neg(Relaxed);

    assert_eq!(some_var.load(Relaxed), -885.0);

    let some_var = atomic_float::AtomicF64::new(800.0f64);
    some_var.fetch_add(30.0, Relaxed);
    some_var.fetch_sub(-55.0, Relaxed);
    some_var.fetch_neg(Relaxed);

    assert_eq!(some_var.load(Relaxed), -885.0);
}

pub fn atomig_example() {
    let some_var = Atomic::new(0);
    some_var.store(800, Relaxed);

    some_var.fetch_add(30, Relaxed);
    some_var.fetch_sub(-55, Relaxed);

    assert_eq!(some_var.load(Relaxed), 885);
}

pub fn atomicbox_examples() {
    let atom = AtomicBox::new(Box::new("one"));
    let mut boxed = Box::new("two");
    atom.swap_mut(&mut boxed, Ordering::AcqRel);
    assert_eq!(*boxed, "one");
}
