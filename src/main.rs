//! A program for driving utilization and starvation of
//! specific system resources.
//!
//! Each subcommand implements a specific type of stress to
//! put on a specific subsystem - see `cg --help` for a list
//! of those commands.

#[macro_use]
extern crate clap;
extern crate cg;

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
                .help("file to write the PID of the current execution to"),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("cpu")
                .about("drive user cpu utilization to the top")
                .arg(
                    Arg::with_name("threads")
                        .default_value("4")
                        .short("t")
                        .long("threads")
                        .help("number of threads to use"),
                ),
        )
        .subcommand(
            SubCommand::with_name("memory")
                .about("tries to allocate a lot of memory")
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
                        .help("number of times to allocate blocks"),
                ),
        )
        .subcommand(
            SubCommand::with_name("memory-wave")
                .about("keeps allocating and deallocating memory in intervals")
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
                        .help("number of times to allocate blocks"),
                )
                .arg(
                    Arg::with_name("interval")
                        .default_value("100")
                        .short("i")
                        .long("interval")
                        .help("time to wait before allocs and deallocs (ms)"),
                ),
        )
        .get_matches();

    write_pid_to_file(matches.value_of("pid").unwrap());

    match matches.subcommand() {
        ("cpu", Some(m)) => {
            cg::cpu::exercise(value_t!(m, "threads", usize).unwrap());
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
