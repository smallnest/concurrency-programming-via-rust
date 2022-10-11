pub mod threads;

pub use threads::*;


pub fn panic_example() {
    println!("Hello, world!");

    let h = std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        panic!("boom");
    });

    let r = h.join();
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e)
    }

    println!("Exiting main!")
}