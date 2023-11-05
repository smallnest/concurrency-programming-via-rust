use std::time::Duration;
use ticker::Ticker;

pub fn ticker_example() {
    let ticker = Ticker::new(0..10, Duration::from_secs(1));
    for i in ticker {
        println!("{:?}", i)
    }
}

pub fn async_io_interval() {
    use async_io::Timer;
    use futures_lite::StreamExt;

    let mut count = 0;

    smol::block_on(async {
        let mut tick = Timer::interval(Duration::from_secs(1));

        while let Some(_) = tick.next().await {
            println!("第{}秒", count);
            count += 1;

            if count >= 10 {
                break;
            }
        }
    });
}
