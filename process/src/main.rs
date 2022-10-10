use std::process::{Command, Stdio};
use process::*;

fn main() {
    spawn_a_process();
    process_io();
    child();
    kill();

    async_process_example();
    process_control_example();
    easy_process_example();
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

    let sed_child = Command::new("sed")
        .arg("s/tpyo/typo/")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start sed process");

    let output = sed_child.wait_with_output().expect("Failed to wait on sed");
    assert_eq!(b"Oh no, a typo!\n", output.stdout.as_slice());
}

fn child() {
    let mut child = Command::new("/bin/cat")
        .arg("Cargo.toml")
        .spawn()
        .expect("failed to execute child");

    let ecode = child.wait().expect("failed to wait on child");

    assert!(ecode.success());
}

fn kill() {
    let mut command = Command::new("yes");
    if let Ok(mut child) = command.spawn() {
        println!("Child's ID is {}", child.id());
        child.kill().expect("command wasn't running");
    } else {
        println!("yes command didn't start");
    }
}
