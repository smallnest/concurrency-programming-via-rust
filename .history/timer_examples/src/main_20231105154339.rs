use timer_examples::*;

fn main() {
    timer_schedule_with_delay();
    timer_schedule_with_date();
    timer_repeat();

    safina_timer_example();

    futures_timer_example();

    async_io_timer_example();
    async_io_interval();
}
