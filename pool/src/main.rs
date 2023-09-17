use pool::*;

fn main() {
    rayon_threadpool();
    threadpool_example();
    scoped_threadpool();
    rusty_pool_example();
    fast_threadpool_example().unwrap();
    scheduled_thread_pool();
    workerpool_rs_example();
    poolite_example();
    poolite_example2();
    executor_service_example();
    threadpool_executor_example();
    executors_example();
}
