use std::sync::Arc;
use std::thread;
use std::time::Duration;

use buffer::Read;
use buffer::Write;

pub mod buffer;

fn main() {
    let buffer = Arc::new(buffer::Buffer::<u8>::new());

    thread::spawn(|| {
        for i in 1..10 {
            let mut buffer = buffer.clone();
            let random_sleep_time = Duration::from_millis((i * 100) as u64);
            thread::sleep(random_sleep_time);

            buffer.write(1);
        }
    });
}
