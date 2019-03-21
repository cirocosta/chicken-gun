//! Filesystem related operations.

use std::{fs, path, thread, time};

/// Creates and keep open a bunch of files.
///
/// - files get created under `directory` and *not* cleaned afterward.
/// - files remain open until the end of the execution (once the process
///   gets signalled).
///
pub fn exercise_files_open(num: usize, directory: &str) {
    let mut open_files = Vec::with_capacity(num);

    for idx in 0..num {
        let filepath = path::Path::new(directory).join(idx.to_string());

        let file = match fs::File::create(&filepath) {
            Err(why) => panic!("failed to create {}: {}", filepath.display(), why),
            Ok(file) => file,
        };

        open_files.push(file);
    }

    thread::sleep(time::Duration::from_secs(1 << 16))
}
