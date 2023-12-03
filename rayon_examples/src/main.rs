
use rayon_examples::*;

fn main() {
    rayon_par_iter();

    rayon_scope_example();
    rayon_scope_example2();
    rayon_scopefifo_example();
    rayon_threadpool_example();
    rayon_global_thread_pool_example();
}
