use sync_cow::SyncCow;
use std::sync::Arc;
use std::any::Any;


pub fn sync_cow_example() -> Result<(),Box<dyn Any + Send>> {
    let cow = Arc::new(SyncCow::new(5));

    // Arc is only needed to pass the ref to the threads
    let cow_write_arc = cow.clone();
    let cow_read_arc = cow.clone();
    let cow_result_arc = cow.clone();

    let writer = std::thread::spawn(move || {
        let cow = &*cow_write_arc; // unpack immediately to avoid Arc deref
        let mut val = 0;
        cow.edit(|x| {
            val = *x;
            *x = 4;
        });
        println!("Cow was {} when writing", val);
    });

    let reader = std::thread::spawn(move || {
        let cow = &*cow_read_arc; // unpack immediately to avoid Arc deref
        println!("Cow was {} when reading", cow.read());
    });

    writer.join()?;
    reader.join()?;

    let cow = &*cow_result_arc; // unpack immediately to avoid Arc deref
    println!("Cow was {} when result", cow.read());

    Ok(())
}