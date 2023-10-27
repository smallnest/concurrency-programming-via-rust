use process::*;

fn main() {
    spawn_a_process();
    process_io();
    child();
    kill();
    pipe();

    async_process_example();
    process_control_example();
    easy_process_example();
}

