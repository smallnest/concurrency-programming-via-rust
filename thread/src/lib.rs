pub mod threads;

pub use threads::*;


pub fn panic_example() {
    println!("Hello, world!");

    let h = thread::spawn(|| {
        thread::sleep_ms(1000);
        panic!("boom");
    });

    let r = h.join();
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e)
    }

    println!("Exiting main!")
}