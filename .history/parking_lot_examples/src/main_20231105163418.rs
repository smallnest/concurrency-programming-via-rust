use parking_lot_examples::*;

fn main() {
    mutex_example();
    mutex_example2();
    mutex_example3();
    mutex_example4();
    
    fairmutex_example();

    rwmutex_example();

    reentrantmutex_example();

    once_example();

    condvar_example();
}
