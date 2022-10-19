use special::*;


fn main() {
    process_lock();

    try_lock_example1();

    sharded_slab_read();
    sharded_slab_write();
    sharded_slab_pool();
    slab_example();

    event_listener_example();
    triggered_example();

    hashmap_example();
    flurry_hashmap();
    flurry_hashset();
    evmap_example();

    concurrent_queue_example();

    async_lock_mutex();
    async_lock_rwlock();
    async_lock_barrier();
    async_lock_semaphore();

    portable_atomic_i128();
    portable_atomic_u128();
    portable_atomic_f64();
    atomic_float_example();

    simple_mutex_example();

    oneshot_example();
    async_oneshot_example();
    catty_example();

    waitgroup_example();
    wg_example();

}


// lockfree