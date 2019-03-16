use std::{mem, thread, time};

/// Allocates `count` blocks of size `block_size` in memory
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

/// Just like `exercise`, it allocates `count * block_size` number
/// of bytes in memory, but:
///
/// - it doesn't do it all at once as fast as it can, instead, it
///   slowly allocates a new block based on `interval`; and
/// - once it reaches the desired size, it deallocates the memory
///   in chunks as well.
///
/// In summary, memory utilization (working set) should look like:
///
/// ```txt
///  
///  alloc       alloc
///
///
///      .--.     .--.
///   __/    \___/    \__
///
///        dealloc   dealloc ...
///
/// ```
///
pub fn exercise_like_a_wave(block_size: usize, count: usize, interval: time::Duration) {
    let mut ptrs = Vec::with_capacity(count);

    loop {
        for _ in 0..count {
            ptrs.push(alloc(block_size));
            thread::sleep(interval);
        }

        for _ in 0..count {
            match ptrs.pop() {
                Some(p) => dealloc(p, block_size),
                None => panic!("unexpected"),
            }

            thread::sleep(interval);
        }
    }
}

fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

fn dealloc(ptr: *mut u8, old_size: usize) {
    unsafe {
        Vec::from_raw_parts(ptr, 0, old_size);
    }
}
