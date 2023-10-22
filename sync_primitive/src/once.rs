#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::sync::Once;
use std::sync::OnceLock;
use std::cell::OnceCell;
use std::thread::sleep;
use std::time::Duration;

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

pub fn oncecell_example() {
	let cell = OnceCell::new();
	assert!(cell.get().is_none());

	let value: &String = cell.get_or_init(|| {
		"Hello, World!".to_string()
	});
	assert_eq!(value, "Hello, World!");
	assert!(cell.get().is_some());

	println!("OnceCell: {}", cell.get().is_some())
}

pub fn oncelock_example() {
	static CELL: OnceLock<String> = OnceLock::new();
	assert!(CELL.get().is_none());

	std::thread::spawn(|| {
		let value: &String = CELL.get_or_init(|| {
			"Hello, World!".to_string()
		});
		assert_eq!(value, "Hello, World!");
	}).join().unwrap();


	sleep(Duration::from_secs(1));

	let value: Option<&String> = CELL.get();
	assert!(value.is_some());
	assert_eq!(value.unwrap().as_str(), "Hello, World!");

	println!("OnceLock: {}", value.is_some())
}

use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

pub fn once_cell_example() {
	println!("{:?}", GLOBAL_DATA.lock().unwrap());
}