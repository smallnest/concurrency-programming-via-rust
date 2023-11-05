use ticker::Ticker;
use std::time::Duration;

pub fn ticker_example() {
    let ticker = Ticker::new(0..10, Duration::from_secs(1));
    for i in ticker {
        println!("{:?}", i)
    }
}


pub fn smol_timer_example2() {use smol::{Timer, stream::Interval};
use std::time::Duration;
  let mut count = 0;

  smol::block_on(async {
    let mut timer = Interval::every(Duration::from_secs(1));

    while let Some(_) = timer.next().await {
      println!("第{}秒", count);
      count += 1;
      
      if count >= 10 {
        break;
      }
    }
  });
}
