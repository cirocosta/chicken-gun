use std::process;

#[cfg(target_os = "linux")]
pub fn exercise(procs_num: usize) {
    let mut children = Vec::with_capacity(procs_num);

    for _ in 0..procs_num {
        children.push(
            process::Command::new("/proc/self/exe")
                .arg("sleep")
                .spawn()
                .expect("failed to execute child"),
        );
    }

    for mut child in children {
        child.wait().expect("failed waiting for child");
    }
}
