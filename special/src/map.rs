use dashmap::DashMap;
use std::sync::Arc;
use std::thread;

pub fn hashmap_example() {
    let map = Arc::new(DashMap::new());

    let map1 = map.clone();
    let whandle = thread::spawn(move || {
        map1.insert(1, 2);
        map1.insert(2, 3);
    });

    let map2 = map.clone();
    let rhandle = thread::spawn(move || {
        loop {
            if let Some(v) = map2.get(&1) {
                println!("get value {} for key 1", *v);
                break;
            }
        }

        loop {
            if let Some(v) = map2.get(&2) {
                println!("get value {} for key 2", *v);
                break;
            }
        }
    });

    whandle.join().unwrap();
    rhandle.join().unwrap();
}

pub fn flurry_hashmap() {
    let map = flurry::HashMap::new();

    assert_eq!(map.pin().insert(37, "a"), None);
    assert_eq!(map.pin().is_empty(), false);
}

pub fn flurry_hashset() {
    // Initialize a new hash set.
    let books = flurry::HashSet::new();
    let guard = books.guard();

    // Add some books
    books.insert("Fight Club", &guard);
    books.insert("Three Men In A Raft", &guard);
    books.insert("The Book of Dust", &guard);
    books.insert("The Dry", &guard);

    // Check for a specific one.
    if !books.contains(&"The Drunken Botanist", &guard) {
        println!("We don't have The Drunken Botanist.");
    }

    // Remove a book.
    books.remove(&"Three Men In A Raft", &guard);

    // Iterate over everything.
    for book in books.iter(&guard) {
        println!("{}", book);
    }
}

pub fn evmap_example() {
    let (book_reviews_r, mut book_reviews_w) = evmap::new();
    
    let readers: Vec<_> = (0..4)
        .map(|_| {
            let r = book_reviews_r.clone();
            thread::spawn(move || {
                loop {
                    let l = r.len();
                    if l == 0 {
                        thread::yield_now();
                    } else {
                        // the reader will either see all the reviews,
                        // or none of them, since refresh() is atomic.
                        assert_eq!(l, 4);
                        break;
                    }
                }
            })
        })
        .collect();

    // do some writes
    book_reviews_w.insert("Adventures of Huckleberry Finn", "My favorite book.");
    book_reviews_w.insert("Grimms' Fairy Tales", "Masterpiece.");
    book_reviews_w.insert("Pride and Prejudice", "Very enjoyable.");
    book_reviews_w.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.");
    // expose the writes
    book_reviews_w.refresh();

    // you can read through the write handle
    assert_eq!(book_reviews_w.len(), 4);

    // the original read handle still works too
    assert_eq!(book_reviews_r.len(), 4);

    // all the threads should eventually see .len() == 4
    for r in readers.into_iter() {
        assert!(r.join().is_ok());
    }
}
