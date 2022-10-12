// Box<T>, casually referred to as a ‘box’, provides the simplest form of heap allocation in Rust. 
// Boxes provide ownership for this allocation, and drop their contents when they go out of scope. 
// Boxes also ensure that they never allocate more than isize::MAX bytes.
pub fn box_example() {
    let b = Box::new(5);
    println!("b = {}", b);
}

pub fn box_example2() {
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }

    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}

