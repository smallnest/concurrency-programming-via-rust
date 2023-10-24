use collections::*;    

fn main() {
    common_thread_safe_collections();
    common_thread_safe_vec();
    common_thread_safe_linkedlist();
    
    dashmap_example();
    cuckoofilter_example();
    evmap_example();
}
