use sync_primitive::*;

fn main() {
    arc_example();
    arc_example2();
    arc_example3();

    mutex_example1();
    mutex_example2_poison();
    mutex_example3_drop();

    rwlock_example();

    once_example();

    barrier_example();

    condvar_example();
    condvar_example2();

    mpsc_example();
    sync_channel_example();

    atomic_example();
    atomic_example2();
}
