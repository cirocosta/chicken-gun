extern crate libc;
extern crate rand;

use rand::Rng;
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

pub fn context_switches(thread_num: usize, cpu_num: usize) {
    let mut child_threads = vec![];

    for _ in 0..thread_num {
        child_threads.push(thread::spawn(move || context_switcher(cpu_num)))
    }

    for child in child_threads {
        let _ = child.join();
    }
}

fn context_switcher(cpu_num: usize) {
    let mut rng = rand::thread_rng();

    loop {
        let mut cpu = rng.gen_range(0, cpu_num);
        let mut iterations = 1 << 8;

        // 0 .. 1 .. 2 .. 3
        while iterations > 0 {
            set_affinity(cpu);
            cpu = (cpu + 1) % cpu_num;
            iterations -= 1;
        }
    }
}

fn set_affinity(cpu: usize) {
    unsafe {
        let mut cpuset: libc::cpu_set_t = std::mem::zeroed();

        libc::CPU_ZERO(&mut cpuset);
        libc::CPU_SET(cpu, &mut cpuset);

        libc::sched_setaffinity(0, libc::CPU_SETSIZE as usize, &cpuset);
    }
}
