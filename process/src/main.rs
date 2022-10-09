use std::process::{Command, Stdio};

fn main() {
    spawn_a_process();
    process_io();
}

fn spawn_a_process() {
    let output = Command::new("echo")
        .arg("Hello world")
        .output()
        .expect("Failed to execute command");

    assert_eq!(b"Hello world\n", output.stdout.as_slice());
}

fn process_io() {
    let echo_child = Command::new("echo")
        .arg("Oh no, a tpyo!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");

    let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

    let mut sed_child = Command::new("sed")
        .arg("s/tpyo/typo/")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start sed process");

    let output = sed_child.wait_with_output().expect("Failed to wait on sed");
    assert_eq!(b"Oh no, a typo!\n", output.stdout.as_slice());
}
