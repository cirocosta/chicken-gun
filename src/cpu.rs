extern crate libc;
extern crate rand;

use rand::Rng;
use std::thread;

/// Puts `thread_num` threads to work as hard as they can without
/// preferences in terms of which CPU to run.
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

/// Puts `thread_num` threads to switch around `cpu_num` cpus
/// by switching the `cpu` affinity all the time.
///
/// This has the effect of driving a huge number context switches
/// for the task, while leaving the number of context switches of
/// the parent task almost zero.
///
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

        while iterations > 0 {
            set_affinity(cpu);
            cpu = (cpu + 1) % cpu_num;
            iterations -= 1;
        }
    }
}

/// Ties the current thread to a particular `cpu`.
///
/// Under the hood, a mask is created on the stack and
/// passed down to the kernel to indicate in which set of
/// CPUs the task should be allowed to run.
///
/// If the thread is not running in the indicated CPU, then
/// the thread is migrated to the one set in the mask.
fn set_affinity(cpu: usize) {
    unsafe {
        let mut cpuset: libc::cpu_set_t = std::mem::zeroed();

        libc::CPU_ZERO(&mut cpuset);
        libc::CPU_SET(cpu, &mut cpuset);

        libc::sched_setaffinity(0, libc::CPU_SETSIZE as usize, &cpuset);
    }
}
