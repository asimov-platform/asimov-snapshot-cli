// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]

use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::clap::{Parser, Subcommand},
};

/// ASIMOV Snapshot Command-Line Interface (CLI)
#[derive(Debug, Parser)]
#[command(name = "asimov-snapshot", long_about)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {}

pub fn main() -> SysexitsError {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let Ok(args) = clientele::args_os() else {
        return EX_USAGE;
    };

    // Parse command-line options:
    let options = Options::parse_from(&args);

    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    // Print the version, if requested:
    if options.flags.version {
        println!("asimov-snapshot {}", env!("CARGO_PKG_VERSION"));
        return EX_OK;
    }

    // Print the license, if requested:
    if options.flags.license {
        print!("{}", include_str!("../UNLICENSE"));
        return EX_OK;
    }

    // Execute the given command:
    let result = match options.command.unwrap() {};

    match result {
        Ok(()) => EX_OK,
        Err(err) => err,
    }
}
