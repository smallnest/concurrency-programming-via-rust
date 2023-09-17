use tokio_examples::*;

fn main() {
    process();
    process2();

    oneshot();
    async_with_oneshot();
    mpsc_example();
    broadcast_example();
    watch_example();

    barrier_example();
    mutex_example();
    rwlock_example();
    semaphore_example();
    semaphore_example2();
    notify_example();
    notify_example2();
    
    tokio_rayon_example();
}
