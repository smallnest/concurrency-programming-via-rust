use crossbeam_examples::*;
fn main() {
    atomic_cell_example();

    crossbeam_deque_example();  
    arrayqueue_example();
    segqueue_example();
    
    unbounded_channel_example();
    bounded_channel_example();
    channel_iter_example();
    channel_select_example();
    channel_extra_example();
    sharded_lock_example();
    waitgroup_example();
    parker_example();

    backoff_example();
    cachepadded_example();
    scope_example();
}
