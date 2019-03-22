//! A program for driving utilization and starvation of
//! specific system resources.
//!
//! Each subcommand implements a specific type of stress to
//! put on a specific subsystem - see `cg --help` for a list
//! of those commands.

#[macro_use]
extern crate clap;
extern crate cg;
extern crate num_cpus;

use clap::{App, AppSettings, Arg, SubCommand};
use std::io::prelude::*;
use std::{fs, path, process, time};

fn write_pid_to_file(filepath: &str) {
    let path = path::Path::new(filepath);
    let mut file = match fs::File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    if let Err(e) = file.write_all(process::id().to_string().as_bytes()) {
        panic!("couldn't write pid to file {}: {}", path.display(), e);
    };
}

fn main() {
    let matches = App::new("chicken-gun")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("pid")
                .default_value("/tmp/cg.pid")
                .short("p")
                .long("pid")
                .help("File to write the PID of the current execution to"),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("sleep")
                .setting(AppSettings::Hidden)
                .about("Does nothing - just sleeps until a signal arrives"),
        )
        .subcommand(
            SubCommand::with_name("tcp-receiver")
                .about("Sets up a TCP server that writes the contents received to a file")
                .arg(
                    Arg::with_name("address")
                        .default_value("127.0.0.1:1337")
                        .short("a")
                        .long("address")
                        .help("Address to bind to"),
                )
                .arg(
                    Arg::with_name("destination")
                        .default_value("/dev/null")
                        .short("d")
                        .long("destination")
                        .help("File to write to"),
                ),
        )
        .subcommand(
            SubCommand::with_name("tcp-transmitter")
                .about("Sets up a TCP client that sends a file to the server")
                .arg(
                    Arg::with_name("address")
                        .default_value("127.0.0.1:1337")
                        .short("a")
                        .long("address")
                        .help("Address to connect to"),
                )
                .arg(
                    Arg::with_name("source")
                        .default_value("/dev/zero")
                        .short("s")
                        .long("source")
                        .help("File to read contents from"),
                ),
        )
        .subcommand(
            SubCommand::with_name("files-open")
                .about("Creates and opens a bunch of files")
                .arg(
                    Arg::with_name("number")
                        .default_value("30")
                        .short("n")
                        .long("number")
                        .help("Number of processes to create"),
                )
                .arg(
                    Arg::with_name("directory")
                        .default_value("./")
                        .short("d")
                        .long("directory")
                        .help("Where to create the files to open"),
                ),
        )
        .subcommand(
            SubCommand::with_name("disk-consistent-writes")
                .about("Creates files and then writes to them nonstop")
                .arg(
                    Arg::with_name("number")
                        .default_value("30")
                        .short("n")
                        .long("number")
                        .help("Number of processes to create"),
                )
                .arg(
                    Arg::with_name("directory")
                        .default_value("./")
                        .short("d")
                        .long("directory")
                        .help("Where to create the files to open"),
                ),
        )
        .subcommand(
            SubCommand::with_name("pids")
                .about("Create a bunch of processes")
                .arg(
                    Arg::with_name("number")
                        .default_value("30")
                        .short("n")
                        .long("number")
                        .help("Number of processes to create"),
                ),
        )
        .subcommand(
            SubCommand::with_name("cpu")
                .about("Drive user cpu utilization to the top")
                .arg(
                    Arg::with_name("threads")
                        .default_value("4")
                        .short("t")
                        .long("threads")
                        .help("Number of threads to use"),
                ),
        )
        .subcommand(
            SubCommand::with_name("context-switches")
                .about("Drive context switches to the top")
                .arg(
                    Arg::with_name("threads")
                        .default_value("4")
                        .short("t")
                        .long("threads")
                        .help("Number of threads to use"),
                ),
        )
        .subcommand(
            SubCommand::with_name("memory")
                .about("Tries to allocate a lot of memory")
                .arg(
                    Arg::with_name("bs")
                        .long("bs")
                        .default_value("1024")
                        .help("size of the blocks to allocate"),
                )
                .arg(
                    Arg::with_name("count")
                        .long("count")
                        .default_value("1024")
                        .help("Number of times to allocate blocks"),
                ),
        )
        .subcommand(
            SubCommand::with_name("memory-wave")
                .about("Keeps allocating and deallocating memory in intervals")
                .arg(
                    Arg::with_name("bs")
                        .long("bs")
                        .default_value("1024")
                        .help("Size of the blocks to allocate"),
                )
                .arg(
                    Arg::with_name("count")
                        .long("count")
                        .default_value("1024")
                        .help("Number of times to allocate blocks"),
                )
                .arg(
                    Arg::with_name("interval")
                        .default_value("100")
                        .short("i")
                        .long("interval")
                        .help("Time to wait before allocs and deallocs (ms)"),
                ),
        )
        .get_matches();

    if !matches.is_present("sleep") {
        write_pid_to_file(matches.value_of("pid").unwrap());
    }

    match matches.subcommand() {
        ("sleep", Some(_m)) => {
            std::thread::sleep(std::time::Duration::from_secs(1 << 32));
        }

        ("tcp-receiver", Some(m)) => {
            cg::net::tcp_receiver(
                &value_t!(m, "address", String).unwrap(),
                &value_t!(m, "destination", String).unwrap(),
            );
        }

        ("tcp-transmitter", Some(m)) => {
            cg::net::tcp_transmitter(
                &value_t!(m, "address", String).unwrap(),
                &value_t!(m, "source", String).unwrap(),
            );
        }

        ("files-open", Some(m)) => {
            cg::fs::exercise_files_open(
                value_t!(m, "number", usize).unwrap(),
                &value_t!(m, "directory", String).unwrap(),
            );
        }

        ("disk-consistent-writes", Some(m)) => {
            cg::fs::exercise_constant_write_throughput(
                value_t!(m, "number", usize).unwrap(),
                &value_t!(m, "directory", String).unwrap(),
            );
        }

        ("cpu", Some(m)) => {
            cg::cpu::exercise(value_t!(m, "threads", usize).unwrap());
        }

        ("pids", Some(m)) => {
            cg::fork::exercise(value_t!(m, "number", usize).unwrap());
        }

        ("context-switches", Some(m)) => {
            cg::cpu::context_switches(value_t!(m, "threads", usize).unwrap(), num_cpus::get());
        }

        ("memory", Some(m)) => {
            cg::memory::exercise(
                value_t!(m, "bs", usize).unwrap(),
                value_t!(m, "count", usize).unwrap(),
            );
        }

        ("memory-wave", Some(m)) => {
            cg::memory::exercise_like_a_wave(
                value_t!(m, "bs", usize).unwrap(),
                value_t!(m, "count", usize).unwrap(),
                time::Duration::from_millis(value_t!(m, "interval", u64).unwrap()),
            );
        }

        ("", None) => println!("No subcommand specified. Check --help."),

        _ => unreachable!(),
    }
}
