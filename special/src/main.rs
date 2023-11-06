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
    barrage_example();
    
    hashmap_example();
    flurry_hashmap();
    flurry_hashset();
    evmap_example();

    concurrent_queue_example();
    triple_buffer_example();

    async_lock_mutex();
    async_lock_rwlock();
    async_lock_barrier();

    portable_atomic_i128();
    portable_atomic_u128();
    portable_atomic_f64();
    atomic_float_example();
    atomig_example();
    atomicbox_examples();

    simple_mutex_example();

    oneshot_example();
    async_oneshot_example();
    catty_example();

    waitgroup_example();
    wg_example();
    awaitgroup_example();

    scc_hashmap();
    scc_hashindex();
    scc_treeindex();
    scc_hashset();
    scc_queue();


    async_lock_semaphore();
    async_weighted_semaphore_example();
    tokio_semaphore_example();

    singleflight_example();
    async_singleflight_example();

    sync_cow_example().unwrap();
    arc_swap_example();

    atomic_waker_example();
}


// lockfree