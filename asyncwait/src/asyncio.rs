use async_stream::stream;

use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use futures_lite::AsyncReadExt;

pub fn stream() {
    futures_lite::future::block_on(async {
        let s = stream! {
            for i in 0..3 {
                yield i;
            }
        };

        pin_mut!(s); // needed for iteration

        while let Some(value) = s.next().await {
            println!("got {}", value);
        }
    });
}

pub fn futures_lite_io(){
    futures_lite::future::block_on(async {
        let input: &[u8] = b"hello";
        let mut reader = futures_lite::io::BufReader::new(input);

        let mut contents = String::new();
        reader.read_to_string(&mut contents).await.unwrap();
    });

}


