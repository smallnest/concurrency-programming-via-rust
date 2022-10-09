use primitive::*;
use std::thread;

fn main() {
    cow_example();
    cow_example2();

    box_example();
    box_example2();

    cell_example();
    refcell_example();
    rc_refcell_example();
    once_cell_example();
    lazy_cell_example();
    myrc_example();

    rc_example();
}