#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::sync::Once;


pub fn once_example() {
	let once = Once::new();
	let mut val: usize = 0;

	once.call_once(|| {
		val = 100;
	});

	if once.is_completed() {
		println!("Once: {}", val);
	}
	
}