use event_listener::Event;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn event_listener_example() {
    let flag = Arc::new(AtomicBool::new(false));
    let event = Arc::new(Event::new());

    // Spawn a thread that will set the flag after 1 second.
    thread::spawn({
        let flag = flag.clone();
        let event = event.clone();
        move || {
            // Wait for a second.
            thread::sleep(Duration::from_secs(1));

            // Set the flag.
            flag.store(true, Ordering::SeqCst);

            // Notify all listeners that the flag has been set.
            event.notify(usize::MAX);
        }
    });

    // Wait until the flag is set.
    loop {
        // Check the flag.
        if flag.load(Ordering::SeqCst) {
            break;
        }

        // Start listening for events.
        let listener = event.listen();

        // Check the flag again after creating the listener.
        if flag.load(Ordering::SeqCst) {
            break;
        }

        // Wait for a notification and continue the loop.
        listener.wait();
    }

    println!("flag is set");
}

pub fn triggered_example() {
    smol::block_on(async {
        let (trigger, listener) = triggered::trigger();

        let task = smol::spawn(async {
            // Blocks until `trigger.trigger()` below
            listener.await;

            println!("Triggered async task");
        });

        // This will make any thread blocked in `Listener::wait()` or async task awaiting the
        // listener continue execution again.
        trigger.trigger();

        let _ = task.await;
    })
}

pub fn barrage_example() {
    smol::block_on(async {
        let (tx, rx) = barrage::unbounded();
        let rx2 = rx.clone();
        tx.send_async("Hello!").await.unwrap();
        assert_eq!(rx.recv_async().await, Ok("Hello!"));
        assert_eq!(rx2.recv_async().await, Ok("Hello!"));
    });
}
