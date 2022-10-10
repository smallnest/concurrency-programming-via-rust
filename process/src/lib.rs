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
