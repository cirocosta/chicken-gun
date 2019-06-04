use std::path::Path;
use std::io::Read;

/// Fills up the page cache and then allocates memory.
///
/// 1. fill up the page cache
///     - reads from a bunch of small files that live in a
///       directory
///
pub fn exercise (dir: &Path) {
    let bytes_read = read_files_from_dir(dir);
    println!("read: {}", pretty_bytes::converter::convert(bytes_read as f64));
}

/// Size of the buffers allocated to receive bytes read from files.
///
const BUF_SIZE: usize = 4096;

fn read_files_from_dir(dir: &Path) -> usize {
    if !dir.is_dir() {
        panic!("{} is not a directory", dir.display())
    }

    let mut acc: usize = 0;

    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir () {
            continue
        }

        acc += read_and_discard(&path);
    }

    acc
}

/// Reads all of the contents of a file, discarding what was read.
///
fn read_and_discard(filepath: &Path) -> usize {
    let mut f = std::fs::File::open(filepath).unwrap();
    let mut buf = vec![0u8; BUF_SIZE];
    let mut acc: usize = 0;

    loop {

        match f.read(&mut buf) {
            Ok(0) => return acc,
            Ok(n) => acc += n,
            Err(e) => panic!("failed to read from file {}: {}", filepath.display(), e)
        }
    }
}
