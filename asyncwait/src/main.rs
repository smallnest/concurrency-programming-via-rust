use asyncwait::*;

fn main() {
    tokio_async();
    futures_async();
    futures_lite_async();
    async_std();
    async_std_task();
    smol_async();

    timefuture_async();
    
    try_join();
    join();
    select();
    futures_select();
    smol_zip();

    stream();

    kviterator_example();

    async_trait_example();

    match monoio_example() {
        Ok(_) => println!("monoio_example: Ok"),
        Err(e) => println!("monoio_example: Err: {}", e),
    }
}
