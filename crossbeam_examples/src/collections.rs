use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::iter;

use crossbeam::deque::{Injector, Steal, Stealer, Worker};
use crossbeam::queue::{ArrayQueue,SegQueue};

pub fn crossbeam_deque_example() {
    let worker = Worker::new_fifo();
    let stealer = worker.stealer();

    // test worker and stealer
    let worker_handle = thread::spawn(move || {
        worker.push(1);
        worker.push(2);
        worker.push(3);
        worker.push(4);
        worker.push(5);
    });

    let stealer_handle = thread::spawn(move || {
        assert_eq!(stealer.steal(), Steal::Success(1));
        assert_eq!(stealer.steal(), Steal::Success(2));
        assert_eq!(stealer.steal(), Steal::Success(3));
        assert_eq!(stealer.steal(), Steal::Success(4));
        assert_eq!(stealer.steal(), Steal::Success(5));
        assert_eq!(stealer.steal(), Steal::Empty);
    });

    worker_handle.join().unwrap();
    stealer_handle.join().unwrap();

    // test global queue (injector) and worker
    let worker = Worker::new_fifo();
    let injector = Arc::new(Mutex::new(Injector::new()));

    // test worker and stealer
    let injector1 = injector.clone();
    let injector_handle = thread::spawn(move || {
        let injector = injector1.lock().unwrap();
        injector.push(1);
        injector.push(2);
        injector.push(3);
        injector.push(4);
        injector.push(5);
    });
    injector_handle.join().unwrap();

    let injector2 = injector.clone();
    let global_handle = thread::spawn(move || {
        let injector = injector2.lock().unwrap();

        // steal the half of the data
        let _ = injector.steal_batch(&worker);
        assert_eq!(worker.pop(), Some(1));
        assert_eq!(worker.pop(), Some(2));
        assert_eq!(worker.pop(), Some(3));

        let _ = injector.steal_batch(&worker);
        assert_eq!(worker.pop(), Some(4));
        let _ = injector.steal_batch(&worker);
        assert_eq!(worker.pop(), Some(5));
        let _ = injector.steal_batch(&worker);
        assert_eq!(worker.pop(), None);
    });

    global_handle.join().unwrap();


    // find_task
    let worker1 = Worker::new_fifo();
    let stealer1 = worker1.stealer();
    let worker2 = Worker::new_fifo();
    let stealer2 = worker2.stealer();
    let stealers = vec![stealer1,stealer2];
    let injector = Arc::new(Mutex::new(Injector::new()));

    let worker_handle = thread::spawn(move || {
        worker1.push(1);
        worker1.push(2);
        worker1.push(3);
        worker1.push(4);
        worker1.push(5);

        assert_eq!(worker1.pop(), Some(1));
    });
    worker_handle.join().unwrap();


    let injector1 = injector.clone();
    let worker2_handle = thread::spawn(move || {
        assert_eq!(find_task(&worker2, injector.clone(), &stealers[..]), Some(2));
        assert_eq!(find_task(&worker2, injector1.clone(), &stealers[..]), Some(3));
        assert_eq!(find_task(&worker2, injector1.clone(), &stealers[..]), Some(4));
        assert_eq!(find_task(&worker2, injector1.clone(), &stealers[..]), Some(5));
    });

    worker2_handle.join().unwrap();


}

fn find_task<T>(local: &Worker<T>, global: Arc<Mutex<Injector<T>>>, stealers: &[Stealer<T>]) -> Option<T> {
   
    // Pop a task from the local queue, if not empty.
    local.pop().or_else(|| {
        // Otherwise, we need to look for a task elsewhere.
        iter::repeat_with(|| {
            // Try stealing a batch of tasks from the global queue.
            let global = global.lock().unwrap();
            global
                .steal_batch_and_pop(local)
                // Or try stealing a task from one of the other threads.
                .or_else(|| stealers.iter().map(|s| s.steal()).collect())
        })
        // Loop while no task was stolen and any steal operation needs to be retried.
        .find(|s| !s.is_retry())
        // Extract the stolen task, if there is one.
        .and_then(|s| s.success())
    })
}

pub fn arrayqueue_example() {
    let q = Arc::new(ArrayQueue::new(2));

    let q1 = q.clone();
    let p = thread::spawn(move || {
        q1.push(1).unwrap();
        q1.push(2).unwrap();
    });

    let q2 = q.clone();
    let c = thread::spawn(move || {
        println!("q2 pop: {}", q2.pop().unwrap_or(-1));
        println!("q2 pop: {}", q2.pop().unwrap_or(-1));
    });

    p.join().unwrap();
    c.join().unwrap();
}


pub fn segqueue_example() {
    let q = Arc::new(SegQueue::new());

    let q1 = q.clone();
    let p = thread::spawn(move || {
        q1.push(1);
        q1.push(2);
    });

    let q2 = q.clone();
    let c = thread::spawn(move || {
        println!("SegQueue q2 pop: {}", q2.pop().unwrap_or(-1));
        println!("SegQueue q2 pop: {}", q2.pop().unwrap_or(-1));
    });

    p.join().unwrap();
    c.join().unwrap();
}