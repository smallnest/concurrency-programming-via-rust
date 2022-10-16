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
        Err(e) => println!("Got an error! {:?}", e),
    }

    println!("Exiting main!")
}


pub fn panic_caught_example() {
    println!("Hello, panic_caught_example !");

    let h = std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(1000));
       let result = std::panic::catch_unwind(|| {
            panic!("boom");
        });

        println!("panic caught, result = {}", result.is_err());
    });

    let r = h.join();
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e),
    }

    println!("Exiting main!")
}

