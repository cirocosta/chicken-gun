use std::{thread, time};

/// Allocates `_count` blocks of size `block_size` in memory
/// (heap) as a way of exercising the need for a specific amount
/// of memory.
///
/// More specifically, this code block is expected to put in the heap:
///
/// ```txt
///
///     capacity * sizeof(T) + sizeof(Vec))
///        |          |             |
///        *-----.----*             |
///           bs * count            24
///     
/// ```
///
/// Once the memory is allocated, `exercise` puts the thread
/// to sleep until interrupted.
///
pub fn exercise(block_size: usize, count: usize) {
    let mut buf = Vec::with_capacity(block_size * count);

    for _ in 0..count {
        buf.append(&mut vec![true; block_size]);
    }

    println!("sleeping");
    thread::sleep(time::Duration::from_secs(60 * 10));
}
