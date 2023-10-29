use channel::*;

fn main() {
    mpsc_example1();
    mpsc_example2();
    mpsc_example3();
    mpsc_example4();
    mpsc_drop_example();

    crossfire_mpsc();
    crossfire_mpmc();

    flume_example();
    flume_select();
    flume_async();

    async_channel_example();
    async_priority_channel_example();
    futures_channel_mpsc_example();
    futures_channel_oneshot_example();
    kanal_example();
    kanal_async_example();
    kanal_oneshot_example();
}
