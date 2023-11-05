use ticker::Ticker;
use std::time::Duration;

pub fn ticker_example() {
    let ticker = Ticker::new(0..10, Duration::from_secs(1));
    for i in ticker {
        println!("{:?}", i)
    }
}
