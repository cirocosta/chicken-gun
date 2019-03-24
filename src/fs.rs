//! Filesystem related operations.

use nix::fcntl;
use nix::unistd::pipe;
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
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

/// Copies a file from `src` to `dst` with minimal userspace
/// demands.
pub fn copy_file(src: &str, dst: &str) {
    let dst_file = match fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path::Path::new(dst))
    {
        Err(why) => panic!("couldn't open|create dst file - {}", why),
        Ok(f) => f,
    };

    let src_file = match fs::File::open(path::Path::new(src)) {
        Err(why) => panic!("couldn't source file - {}", why),
        Ok(f) => f,
    };

    in_kernel_copy(&src_file, &dst_file);
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

fn consistent_writes_to_file(filepath: &str, mbs: usize) {
    let chunk = &[0; KB];
    let writes: usize = mbs * MB / KB;

    let mut file = match fs::File::create(&filepath) {
        Err(why) => panic!("failed to create {}: {}", filepath, why),
        Ok(file) => file,
    };

    loop {
        for _ in 0..writes {
            if let Err(err) = file.write_all(chunk) {
                panic!("failed to write chunk to file - {}", err);
            }
        }

        if let Err(err) = file.sync_all() {
            panic!("failed to sync_all - {}", err);
        }

        if let Err(err) = file.seek(std::io::SeekFrom::Start(0)) {
            panic!("failed to seek - {}", err);
        }
    }
}

/// Redirects all of the contents that can be read
/// from the stream into the Linux null device in order
/// to quickly consume all that is written to it.
pub fn in_kernel_copy(src: &AsRawFd, dst: &AsRawFd) {
    let (rd, wr) = pipe().unwrap();
    let bufsize = fcntl::fcntl(rd, fcntl::F_GETPIPE_SZ).unwrap() as usize;

    loop {
        let res = fcntl::splice(
            src.as_raw_fd(),
            None,
            wr,
            None,
            bufsize,
            fcntl::SpliceFFlags::empty(),
        )
        .unwrap();

        if res == 0 {
            break;
        }

        let _res = fcntl::splice(
            rd,
            None,
            dst.as_raw_fd(),
            None,
            bufsize,
            fcntl::SpliceFFlags::empty(),
        )
        .unwrap();
    }
}
