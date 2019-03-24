extern crate nix;

use crate::fs::in_kernel_copy;
use std::{fs, net, path};

/// Listens for TCP connections, consuming all of the
/// contents that gets written to them, throwing that
/// content on the floor (writes to sink).
#[cfg(target_os = "linux")]
pub fn tcp_receiver(address: &str, filepath: &str) {
    let listener = net::TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let file = match fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(path::Path::new(filepath))
        {
            Err(why) => panic!("couldn't open file - {}", why),
            Ok(f) => f,
        };

        in_kernel_copy(&stream.unwrap(), &file);
    }
}

pub fn tcp_transmitter(address: &str, filepath: &str) {
    let file = match fs::File::open(path::Path::new(filepath)) {
        Err(why) => panic!("couldn't open source file - {}", why),
        Ok(f) => f,
    };

    let stream = match net::TcpStream::connect(address) {
        Err(why) => panic!("couldn't connect to address {} - {}", address, why),
        Ok(s) => s,
    };

    in_kernel_copy(&file, &stream);
}
