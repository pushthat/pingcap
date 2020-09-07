use clap::Clap;
use std::process::exit;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Matias B. <matias.berny@icloud.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "0.1.0", author = "Matias B. <matias.berny@icloud.com>")]
    Set(SetParam),
    Get(GetParam),
    Rm(RmParam),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct GetParam {
    /// Print debug info
    key: String,
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct RmParam {
    /// Print debug info
    key: String,
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct SetParam {
    /// Print debug info
    key: String,
    value: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Set(t) => {
            eprintln!("unimplemented");
            exit(1);
        }
        SubCommand::Get(t) => {
            eprintln!("unimplemented");
            exit(1);
        }
        SubCommand::Rm(t) => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
