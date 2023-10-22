use sync_primitive::*;

fn main() {
    arc_example();
    arc_example2();
    arc_example3();

    mutex_example1();
    mutex_example2_poison();
    mutex_example3_drop();
    
    rwlock_example();
    read_after_write();

    once_example();
    oncecell_example();
    oncelock_example();
    once_cell_example();

    barrier_example();
    barrier_recycle_example();

    condvar_example();
    condvar_example2();

    mpsc_example();
    sync_channel_example();

    atomic_example();
    atomic_example2();

    lazy_cell_example();
    lazy_lock_example();

    exclusive_lock_example();
}
