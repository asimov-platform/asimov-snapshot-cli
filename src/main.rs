// This is free and unencumbered software released into the public domain.

#![deny(unsafe_code)]

use asimov_snapshot_cli::commands;
use clientele::{
    StandardOptions,
    SysexitsError::{self, *},
    crates::clap::{Parser, Subcommand},
};
use color_print::ceprintln;

/// ASIMOV Snapshot Command-Line Interface (CLI)
#[derive(Debug, Parser)]
#[command(name = "asimov-snapshot", long_about)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create snapshots for URL(s)
    #[command(external_subcommand)]
    Snapshot(Vec<String>),

    /// List snapshots
    List,

    /// Show log for a URL
    Log {
        /// URL to show log for
        url: String,
    },

    /// Compact snapshots for a URL
    Compact {
        /// URL(s) to compact snapshots for
        urls: Vec<String>,
    },
}

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

    if let Err(err) = std::fs::create_dir_all(asimov_env::paths::asimov_root().join("snapshots"))
        .map_err(|e| {
            ceprintln!("<s,r>error:</> failed to create snapshot directory: {e}");
            EX_IOERR
        })
    {
        return err;
    }

    // Execute the given command:
    let result = match options.command {
        Command::Snapshot(ref urls) => commands::snapshot(urls, &options.flags),
        Command::List => commands::list(&options.flags),
        Command::Log { url } => commands::log(&url, &options.flags),
        Command::Compact { urls } => commands::compact(&urls, &options.flags),
    };

    match result {
        Ok(()) => EX_OK,
        Err(err) => err,
    }
}
