use container_primitive::*;

fn main() {
    cow_example();
    cow_example2();
    beef_cow();

    box_example();
    box_example2();
    thin_box_example();

    cell_example();
    refcell_example();
    rc_refcell_example();
    once_cell_example();
    lazy_cell_example();
    lazy_lock();

    rc_example();
    myrc_example();
}