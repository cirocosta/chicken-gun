extern crate num_cpus;

use std::thread;

pub fn exercise(thread_num: usize) {
    let mut child_threads = vec![];

    for _ in 0..thread_num {
        child_threads.push(thread::spawn(|| loop {}))
    }

    for child in child_threads {
        let _ = child.join();
    }
}

