#[macro_use]
extern crate clap;
extern crate cg;

use clap::{App, AppSettings, Arg, SubCommand};

fn run_memory(_block_size: usize, _count: usize) {
    println!("yay!");
}

fn main() {
    let matches = App::new("chicken-gun")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("cpu")
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

    match matches.subcommand() {
        ("cpu", Some(m)) => cg::cpu::exercise(value_t!(m, "threads", usize).unwrap()),

        ("memory", Some(m)) => {
            run_memory(
                value_t!(m, "bs", usize).unwrap(),
                value_t!(m, "count", usize).unwrap(),
            );
        }

        ("", None) => println!("No subcommand specified. Check --help."),

        _ => unreachable!(),
    }
}
