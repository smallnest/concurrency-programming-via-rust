pub mod primitives;

pub use primitives::*;

use std::process::Stdio;

use tokio;
use tokio::process::Command;
use tokio::io::{BufReader, AsyncBufReadExt};

pub fn process() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let _: Result<(), Box<dyn std::error::Error>> = rt.block_on(async {
        let mut child = Command::new("echo")
            .arg("hello")
            .arg("world")
            .spawn()
            .expect("failed to spawn");

        // Await until the command completes
        let status = child.wait().await?;
        println!("the command exited with: {}", status);

        Ok(())
    });
}

pub fn process2() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let _: Result<(), Box<dyn std::error::Error>> = rt.block_on(async {
        let mut cmd = Command::new("cat");
        cmd.arg("Cargo.toml");

        // Specify that we want the command's standard output piped back to us.
        // By default, standard input/output/error will be inherited from the
        // current process (for example, this means that standard input will
        // come from the keyboard and standard output/error will go directly to
        // the terminal if this process is invoked from the command line).
        cmd.stdout(Stdio::piped());

        let mut child = cmd.spawn().expect("failed to spawn command");

        let stdout = child
            .stdout
            .take()
            .expect("child did not have a handle to stdout");

        let mut reader = BufReader::new(stdout).lines();

        // Ensure the child process is spawned in the runtime so it can
        // make progress on its own while we await for any output.
        tokio::spawn(async move {
            let status = child
                .wait()
                .await
                .expect("child process encountered an error");

            println!("child status was: {}", status);
        });

        while let Some(line) = reader.next_line().await? {
            println!("Line: {}", line);
        }

        Ok(())
    });


}


pub fn oneshot() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let _: Result<(), Box<dyn std::error::Error>> = rt.block_on(async {
        let (tx, rx) = tokio::sync::oneshot::channel::<String>();

        tokio::spawn(async move {
            tx.send("hello".to_string()).unwrap();
        });

        let msg = rx.await?;
        println!("got = {}", msg);

        Ok(())
    });
}

pub fn async_with_oneshot() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    async fn some_computation() -> String {
        "the result of the computation".to_string()
    }

    let _: Result<(), Box<dyn std::error::Error>> = rt.block_on(async {
        let join_handle = tokio::spawn(async move {
            some_computation().await
        });
    
        // Do other work while the computation is happening in the background
    
        // Wait for the computation result
        let res = join_handle.await?;
        println!("result = {}", res);

        Ok(())
    });
}

pub fn mpsc_example() {
    async fn some_computation(input: u32) -> String {
        format!("the result of computation {}", input)
    }
    

    let rt = tokio::runtime::Runtime::new().unwrap();

    let _: Result<(), Box<dyn std::error::Error>> = rt.block_on(async {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);

        tokio::spawn(async move {
            for i in 0..10 {
                let res = some_computation(i).await;
                tx.send(res).await.unwrap();
            }
        });

        while let Some(res) = rx.recv().await {
            println!("got = {}", res);
        }

        Ok(())
    });
}

pub fn broadcast_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let _: Result<(), Box<dyn std::error::Error>> = rt.block_on(async {
        let (tx, mut rx1) = tokio::sync::broadcast::channel::<String>(10);
        let mut rx2 = tx.subscribe();

        tokio::spawn(async move {
            tx.send("hello".to_string()).unwrap();
            tx.send("world".to_string()).unwrap();
        });

        println!("rx1 = {:?}", rx1.recv().await);
        println!("rx2 = {:?}", rx2.recv().await);
        println!("rx2 = {:?}", rx2.recv().await);

        Ok(())
    });
    
}

pub fn watch_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let _: Result<(), Box<dyn std::error::Error>> = rt.block_on(async {
        let (tx, rx1) = tokio::sync::watch::channel::<String>("hello".to_string());
        let mut rx2 = tx.subscribe();

        tokio::spawn(async move {
            tx.send("world".to_string()).unwrap();
        });

        println!("rx1 = {:?}", *rx1.borrow());
        println!("rx2 = {:?}", *rx2.borrow());
        println!("rx2 = {:?}", rx2.changed().await);

        Ok(())
    });

}

/// 实现fib
pub fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }

    return fib(n-1) + fib(n-2);
}

pub fn tokio_rayon_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let nft = tokio_rayon::spawn(|| {
            fib(20)
          }).await;
          
          assert_eq!(nft, 6765);
    })

}