use channel::*;

fn main() {
    mpsc_example1();
    mpsc_example2();
    mpsc_example3();
    mpsc_example4();

    crossfire_mpsc();
    crossfire_mpmc();
    atomic_mpmc();
    flume_example();
    async_channel_example();
    async_priority_channel_example();
    futures_channel_mpsc_example();
    futures_channel_oneshot_example();
}
