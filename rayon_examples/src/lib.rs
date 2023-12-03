use rayon::prelude::*;

pub fn rayon_par_iter() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = v.par_iter().map(|x| x * x).sum::<i32>();
    println!("sum: {}", sum);

    let mut left = vec![42; 10];
    let mut right = vec![-1; 10];

    (10..15)
        .into_par_iter()
        .enumerate()
        .unzip_into_vecs(&mut left, &mut right);

    assert_eq!(left, [0, 1, 2, 3, 4]);
    assert_eq!(right, [10, 11, 12, 13, 14]);
}

pub fn rayon_scope_example() {
    rayon::scope(|s| {
        s.spawn(|_| {
            println!("Hello from a thread!");
        });
    });
}

pub fn rayon_scope_example2() {
    let mut value_a = None;
    let mut value_b = None;
    let mut value_c = None;
    rayon::scope(|s| {
        s.spawn(|s1| {
            value_a = Some(22);

            s1.spawn(|_| {
                value_b = Some(44);
            });
        });

        s.spawn(|_| {
            value_c = Some(66);
        });
    });
    assert_eq!(value_a, Some(22));
    assert_eq!(value_b, Some(44));
    assert_eq!(value_c, Some(66));
}


pub fn rayon_scopefifo_example() {
    rayon::scope_fifo(|s| {
        s.spawn_fifo(|s| { // task s.1
            println!("s.1");
            s.spawn_fifo(|_s| { // task s.1.1
                println!("s.1.1");
                rayon::scope_fifo(|t| {
                    t.spawn_fifo(|_| println!("t.1")); // task t.1
                    t.spawn_fifo(|_| println!("t.2")); // task t.2
                });
            });
        });
        s.spawn_fifo(|_s| { // task s.2
            println!("s.2");
        });
        // point mid
    });
}

pub fn rayon_threadpool_example() {
    fn fib(n: usize) -> usize {
        if n == 0 || n == 1 {
            return n;
        }
        let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of `pool`
        return a + b;
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    let n = pool.install(|| fib(20));

    println!("{}", n);
}

pub fn rayon_global_thread_pool_example() {
        rayon::ThreadPoolBuilder::new().num_threads(22).build_global().unwrap();
}