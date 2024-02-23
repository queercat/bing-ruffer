use std::sync::Arc;
use std::thread;
use std::time::Duration;

use buffer::Read;
use buffer::Write;

pub mod buffer;
pub mod server;

fn main() {
    let buffer = Arc::new(buffer::Buffer::<u8>::new());
}
