extern crate num_cpus;

use std::thread;

/// Puts `n` threads to work as hard as they can without preferences
/// in terms of which CPU to run.
///
/// Once started, they only finish when the main program finishes
/// (usually a SIGINT | SIGTERM).
pub fn exercise(thread_num: usize) {
    let mut child_threads = vec![];

    for _ in 0..thread_num {
        child_threads.push(thread::spawn(|| loop {}))
    }

    for child in child_threads {
        let _ = child.join();
    }
}

