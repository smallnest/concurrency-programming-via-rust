use futures::channel::mpsc;
use futures::executor::{self, ThreadPool};
use futures::try_join;
use futures::StreamExt;
use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};

pub fn tokio_async() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("Hello from tokio!");

        rt.spawn(async {
            println!("Hello from a tokio task!");
            println!("in spawn")
        })
        .await
        .unwrap();
    });

    rt.spawn_blocking(|| println!("in spawn_blocking"));
}

pub fn futures_async() {
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<i32>();

    let fut_values = async {
        let fut_tx_result = async move {
            (0..100).for_each(|v| {
                tx.unbounded_send(v).expect("Failed to send");
            })
        };
        pool.spawn_ok(fut_tx_result);

        let fut_values = rx.map(|v| v * 2).collect();

        fut_values.await
    };

    let values: Vec<i32> = executor::block_on(fut_values);

    println!("Values={:?}", values);
}

pub fn futures_lite_async() {
    futures_lite::future::block_on(async { println!("Hello from futures_lite") })
}

pub fn async_std() {
    async_std::task::block_on(async { println!("Hello from async_std") })
}

pub fn smol_async() {
    smol::block_on(async { println!("Hello from smol") })
}

struct Book();
struct Music();

async fn get_book() -> Result<Book, String> {
    println!("in get_book");
    Ok(Book())
}
async fn get_music() -> Result<Music, String> {
    println!("in get_music");
    Ok(Music())
}
async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book();
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}

pub fn join() {
    futures_lite::future::block_on(async { get_book_and_music().await }).unwrap();
}

pub fn select() {
    futures_lite::future::block_on(async {
        let t1 = get_book().fuse();
        let t2 = get_music().fuse();

        pin_mut!(t1, t2);

        select! {
            _x = t1 => println!("select get_book"),
            _y = t2 => println!("select get_music"),
        }
    });
}

pub fn futures_select() {
    futures_lite::future::block_on(async {
        use futures::future;

        let mut a_fut = future::ready(4);
        let mut b_fut = future::ready(6);
        let mut total = 0;

        loop {
            select! {
                a = a_fut => total += a,
                b = b_fut => total += b,
                complete => {println!("complete"); break},
                default => unreachable!(), // never runs (futures are ready, then complete)
            };
        }
        assert_eq!(total, 10);
    });
}
