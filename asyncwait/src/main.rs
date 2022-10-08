use asyncwait::*;

fn main() {
    tokio_async();
    futures_async();
    futures_lite_async();
    async_std();
    smol_async();

    join();
    select();

    stream();


}
