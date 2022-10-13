use asyncwait::*;

fn main() {
    tokio_async();
    futures_async();
    futures_lite_async();
    async_std();
    smol_async();

    timefuture_async();
    
    join();
    select();
    futures_select();

    stream();

    kviterator_example();

    async_trait_example();
}
