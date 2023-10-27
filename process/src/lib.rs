use std::io;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;

use futures_lite::io::BufReader;
use futures_lite::prelude::*;
use process_control::ChildExt;
use process_control::Control;

pub fn async_process_example() {
    futures_lite::future::block_on(async {
        let mut child = async_process::Command::new("find")
            .arg(".")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();

        while let Some(line) = lines.next().await {
            println!("{}", line.unwrap());
        }
    });
}

pub fn process_control_example() {
    let process = Command::new("echo")
        .arg("hello")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = process
        .controlled_with_output()
        .time_limit(Duration::from_secs(1))
        .terminate_for_timeout()
        .wait()
        .unwrap()
        .ok_or_else(|| io::Error::new(io::ErrorKind::TimedOut, "Process timed out"))
        .unwrap();

    assert_eq!(b"hello", &output.stdout[..5]);
}

pub fn easy_process_example() {
    // stdout
    if let Ok(output) = easy_process::run(r#"sh -c 'echo "1 2 3 4"'"#) {
        assert_eq!(&output.stdout, "1 2 3 4\n");
    }
   

    // stderr
    if let Ok(output) = easy_process::run(r#"sh -c 'echo "1 2 3 4" >&2'"#) {
        assert_eq!(&output.stderr, "1 2 3 4\n");
    }
    
}

pub fn pipe() {
    // 创建两个子进程，一个作为生产者，一个作为消费者

    // 生产者进程
    let producer = Command::new("echo")
        .arg("Hello, Rust!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start producer command");

    // 消费者进程
    let consumer = Command::new("grep")
        .arg("Rust")
        .stdin(producer.stdout.unwrap())
        .output()
        .expect("Failed to start consumer command");

    // 获取消费者的输出
    let output = String::from_utf8_lossy(&consumer.stdout);
    println!("Output: {:?}", output);
}

pub fn spawn_a_process() {
    let output = Command::new("echo")
        .arg("Hello world")
        .output()
        .expect("Failed to execute command");

    assert_eq!(b"Hello world\n", output.stdout.as_slice());
}

pub fn process_io() {
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

pub fn child() {
    let mut child = Command::new("/bin/cat")
        .arg("Cargo.toml")
        .spawn()
        .expect("failed to execute child");

    let ecode = child.wait().expect("failed to wait on child");

    assert!(ecode.success());
}

pub fn kill() {
    let mut command = Command::new("yes");
    if let Ok(mut child) = command.spawn() {
        println!("Child's ID is {}", child.id());
        child.kill().expect("command wasn't running");
    } else {
        println!("yes command didn't start");
    }
}
