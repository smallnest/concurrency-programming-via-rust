use thread::*;

fn main() {
    start_one_thread();
    start_two_threads();
    start_thread_with_sleep();
    start_thread_with_priority();
    thread_builder();

    start_one_thread_with_move();
    start_one_thread_with_thradlocal();

    thread_park();
    thread_park_timeout();

    start_scoped_threads();
}

