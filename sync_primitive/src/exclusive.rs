use std::sync::Exclusive;

pub fn exclusive_lock_example() {
    let mut exclusive = Exclusive::new(92);
    println!("ready");
    std::thread::spawn(move || {
        let counter = exclusive.get_mut();
        println!("{}", *counter);
        *counter = 100;
    }).join().unwrap();

}