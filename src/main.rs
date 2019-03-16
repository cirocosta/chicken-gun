#[macro_use]

extern crate clap;
extern crate num_cpus;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::{process, thread};

fn run(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("user-cpu", Some(m)) => run_user_cpu(m),
        ("memory", Some(m)) => run_memory(m),
        _ => Ok(()),
    }
}

fn run_user_cpu(matches: &ArgMatches) -> Result<(), String> {
    let thread_num = value_t!(matches, "threads", usize).unwrap();
    let mut child_threads = vec![];

    for _ in 0..thread_num {
        child_threads.push(thread::spawn(|| loop {}))
    }

    for child in child_threads {
        let _ = child.join();
    }

    Ok(())
}

fn run_memory(_matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}

fn main() {
    let matches = App::new("chicken-gun")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("user-cpu")
                .about("drive user cpu utilization to the top")
                .arg(
                    Arg::with_name("threads")
                        .default_value("4")
                        .short("t")
                        .help("number of threads to use"),
                ),
        )
        .subcommand(
            SubCommand::with_name("memory")
                .about("tries to allocate a lot of memory")
                .arg(
                    Arg::with_name("bs")
                        .default_value("1024")
                        .help("size of the blocks to allocate"),
                )
                .arg(
                    Arg::with_name("count")
                        .default_value("1024")
                        .help("number of times to allocate blocks"),
                ),
        )
        .get_matches();

    if let Err(e) = run(matches) {
        println!("error: {}", e);
        process::exit(1);
    }
}
