use std::thread;

pub fn oneshot_example() {
    let (sender, receiver) = oneshot::channel::<i32>();
    let sender = thread::spawn(move || {
        sender.send(1).unwrap();
    });
    let receiver = thread::spawn(move || {
        let v = receiver.recv().unwrap();
        println!("get value {}", v);
    });
    sender.join().unwrap();
    receiver.join().unwrap();
}

pub fn async_oneshot_example() {
    let (mut sender, receiver) = async_oneshot::oneshot();
    smol::block_on(async {
        sender.send(1).unwrap();
    });

    smol::block_on(async {
        let v = receiver.try_recv().unwrap();
        println!("get value {}", v);
    });   
}

pub fn catty_example() {
    let (sender, mut receiver) = ::oneshot();
    let sender = thread::spawn(move || {
        sender.send(1).unwrap();
    });
    let receiver = thread::spawn(move || {
        let v = receiver.try_recv().unwrap();
        if v.is_some() {
            println!("get value {}", v.unwrap());
        }
       
    });
    sender.join().unwrap();
    receiver.join().unwrap();
}