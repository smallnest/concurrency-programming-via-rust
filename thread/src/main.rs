use thread::*;

fn main() {
    start_one_thread();
    start_one_thread_result();
    start_two_threads();
    start_n_threads();

    current_thread();
    
    start_thread_with_sleep();
    start_thread_with_yield_now();

    start_thread_with_priority();
    thread_builder();

    start_one_thread_with_move();

    start_threads_with_threadlocal();

    thread_park();
    thread_park2();
    thread_park_timeout();

    start_scoped_threads();
    crossbeam_scope();
    rayon_scope();

    send_wrapper();

    print_thread_amount();

    control_thread();

    #[cfg(not(target_os = "macos"))]
    use_affinity();

    go_thread();

    park_thread();

    panic_example();
    panic_caught_example();

    info();
}
