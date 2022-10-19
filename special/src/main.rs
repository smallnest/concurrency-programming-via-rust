use special::*;


fn main() {
    process_lock();

    try_lock_example1();

    sharded_slab_read();
    sharded_slab_write();
    sharded_slab_pool();
    slab_example();

    event_listener_example();

    hashmap_example();
    concurrent_queue_example();

    async_lock_mutex();
}
