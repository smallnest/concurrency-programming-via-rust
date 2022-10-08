use pool::*;

fn main() {
    rayon_threadpool();
    threadpool_example();
    scoped_threadpool();
    rusty_pool_example();
    fast_threadpool_example().unwrap();
    
    scheduled_thread_pool();

    unblocking_smol().unwrap();
}
