#![feature(lazy_cell)]
#![feature(thin_box)]
#![feature(new_uninit)]
#![feature(cell_update)]

pub mod cow;
pub mod r#box;
pub mod cell;
pub mod rc;

pub use cow::*;
pub use r#box::*;
pub use cell::*;
pub use rc::*;