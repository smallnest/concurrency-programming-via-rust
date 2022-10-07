use thread::*;

fn main() {
    start_one_thread();
    start_two_threads();
    start_thread_with_sleep();
    start_thread_with_priority();
    thread_builder();

    start_one_thread_with_move();
    start_one_thread_with_threadlocal();

    thread_park();
    thread_park_timeout();

    start_scoped_threads();
    crossbeam_scope();
    rayon_scope();

    send_wrapper();

    print_thread_amount();

    control_thread();

    use_affinity();

    go_thread();

    park_thread();
}

