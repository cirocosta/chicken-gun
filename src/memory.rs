use std::{thread,time};

/// Allocates `_count` blocks of size `block_size` in memory
/// as a way of exercising the need for a specific amount of
/// memory (`count` * `block_size` bytes).
///
/// Once the memory is allocated, `exercise` puts the thread
/// to sleep until interrupted.
pub fn exercise(block_size: usize, count: usize) {
    let mut all = Vec::with_capacity(count);

    for _ in 0..count {
        all.push(vec![1;block_size]);
    }

    println!("sleeping");

    thread::sleep(time::Duration::from_secs(60 * 10));
}
