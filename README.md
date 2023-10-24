# concurrency programming via rust

![](book/images/gear.png)

## How to run?

Enter one crate such as thread and run `cargo run`.

## Contents

I have a plan to write a book about conncurrency programming via rust. The below is the contents of it and this repo contains all source codes.

### chapter 1: Thread

Introduces [Threads](thread/src/main.rs) in std and concurrency libs.


### chapter 2: Thread Pool

Introduces [Thread pool](pool/src/main.rs) for std thread.

### chapter 3: async/await

Introduces [async feature](asyncwait/src/main.rs).

### chapter 4: synchronization primitives

Introduces synchronization primitives contains [containers](container_primitive/src/main.rs) 

### chapter 5: basic concurrency primitives
Introduction of basic concurrency [primitives](sync_primitive/src/main.rs) in std lib.

### chapter 6: concurrency collections

Introduces [concurrency collections](collections/src/main.rs) in std lib.

### chapter 7: process

Introduces starting and executing a new [process](process/src/main.rs) in the easy way.

### chapter 8: channel

Introduces each [channels](channel/src/main.rs) such as mpsc, mpmc and broadcasters.

### chapter 9: timer/ticker

Introduces [timer and ticker](timer_examples/src/main.rs).

### chapter 10: parking_lot

Introduces [parking_lot](parking_lot_examples/src/main.rs).

### chapter 11: crossbeam

Introduces [crossbeam](crossbeam_examples/src/main.rs).

### chapter 12: rayon

Introduces [rayon](rayon_examples/src/main.rs).

### chapter 13: tokio

Introduces [tokio](tokio_examples/src/main.rs).

### chapter 14: special

some special synchronization primitives and concurrency libs only for special single purpose.


- replace std::mpsc with crossbeam-channel: https://github.com/rust-lang/rust/pull/93563