use std::boxed::ThinBox;
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

pub fn thin_box_example() {
    use std::mem::{size_of, size_of_val};
    let size_of_ptr = size_of::<*const ()>();

    let box_five = Box::new(5);
    let box_slice = Box::<[i32]>::new_zeroed_slice(5);
    assert_eq!(size_of_ptr, size_of_val(&box_five));
    assert_eq!(size_of_ptr * 2, size_of_val(&box_slice));


    let five = ThinBox::new(5);
    let thin_slice = ThinBox::<[i32]>::new_unsize([1, 2, 3, 4]);
    assert_eq!(size_of_ptr, size_of_val(&five));
    assert_eq!(size_of_ptr, size_of_val(&thin_slice));
}