//! Filesystem related operations.

use std::{fs, path, thread, time};

pub fn exercise_files_open(num: usize, directory: &str) {
    for idx in 0..num {
        let filepath = path::Path::new(directory).join(idx.to_string());

        match fs::File::create(&filepath) {
            Err(why) => panic!("failed to create {}: {}", filepath.display(), why),
            Ok(file) => file,
        };

        // it closes the file for some reason :thinking:
    }

    thread::sleep(time::Duration::from_secs(1 << 16))
}
