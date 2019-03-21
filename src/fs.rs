//! Filesystem related operations.

use std::io::prelude::*;
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

/// Consistently performs writes to a file.
///
/// per thread:
///   while (1):
pub fn exercise_constant_write_throughput(num_files: usize, directory: &str) {
    let mut child_threads = vec![];

    for idx in 0..num_files {
        let filepath = path::Path::new(directory).join(idx.to_string());

        child_threads.push(thread::spawn(move || {
            consistent_writes_to_file(filepath.to_str().unwrap(), 10)
        }))
    }

    for child in child_threads {
        let _ = child.join();
    }
}

const KB: usize = 1 << 10;
const MB: usize = 1 << 20;

///     create file
///     allocate in-memory chunk,
///     for write in writes:
///       write the chunk
///     sync to disk
fn consistent_writes_to_file(filepath: &str, mbs: usize) {
    let chunk = &[0; KB];
    let writes: usize = mbs * MB / KB;

    let mut file = match fs::File::create(&filepath) {
        Err(why) => panic!("failed to create {}: {}", filepath, why),
        Ok(file) => file,
    };

    loop {
        for _ in 0..writes {
            match file.write_all(chunk) {
                Err(why) => panic!("failed to write chunk to file - {}", why),
                Ok(file) => (),
            };
        }

        if let Err(err) = file.sync_all() {
            panic!("failed to sync_all - {}", err);
        }

        if let Err(err) = file.seek(std::io::SeekFrom::Start(0)) {
            panic!("failed to seek - {}", err);
        }
    }
}
