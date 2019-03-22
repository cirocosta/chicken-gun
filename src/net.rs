extern crate nix;

use nix::fcntl;
use nix::unistd::pipe;
use std::os::unix::io::{AsRawFd, RawFd};
use std::{fs, net, path};

const BUFSIZE: usize = 1 << 16;

/// Listens for TCP connections, consuming all of the
/// contents that gets written to them, throwing that
/// content on the floor (writes to sink).
#[cfg(target_os = "linux")]
pub fn tcp_receiver(address: &str, filepath: &str) {
    let listener = net::TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        // TODO - create, instead?
        let file = match fs::File::open(path::Path::new(filepath)) {
            Err(why) => panic!("couldn't open file - {}", why),
            Ok(f) => f,
        };

        let fd_src = stream.unwrap().as_raw_fd();
        let fd_dst = file.as_raw_fd();

        in_kernel_copy(fd_src, fd_dst);
    }
}

pub fn tcp_transmitter(address: &str, filepath: &str) {
    let file = match fs::File::open(path::Path::new(filepath)) {
        Err(why) => panic!("couldn't open null device - {}", why),
        Ok(f) => f,
    };

    let stream = match net::TcpStream::connect(address) {
        Err(why) => panic!("couldn't connect to address {} - {}", address, why),
        Ok(s) => s,
    };

    let fd_src = file.as_raw_fd();
    let fd_dst = stream.as_raw_fd();

    in_kernel_copy(fd_src, fd_dst);
}

/// Determines the size of the buffer that should be used
/// to hold data as it gets transferred from the the socket
/// to the null device.
// fn get_buf_size() -> Result(<usize>) {
// fcntl(2) F_GETPIPE_SZ
// }

/// Redirects all of the contents that can be read
/// from the stream into the Linux null device in order
/// to quickly consume all that is written to it.
fn in_kernel_copy(src: RawFd, dst: RawFd) {
    let (rd, wr) = pipe().unwrap();

    loop {
        let res =
            fcntl::splice(src, None, wr, None, BUFSIZE, fcntl::SpliceFFlags::empty()).unwrap();

        if res == 0 {
            break;
        }

        let _res =
            fcntl::splice(rd, None, dst, None, BUFSIZE, fcntl::SpliceFFlags::empty()).unwrap();
    }
}
