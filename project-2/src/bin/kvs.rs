use clap::Clap;
use kvs::KvStore;
use kvs::KvStorePersist;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Matias B. <matias.berny@icloud.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
    // #[clap(short, long, default_value = "db.db")]
    // config_file: String,
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
    let mut store: KvStorePersist = KvStorePersist::open().expect("err");

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Set(t) => store.set(t.key, t.value).expect("err get"),
        SubCommand::Get(t) => {
            let value = store.get(t.key).expect("").unwrap();
            println!("{}", value);
        }
        SubCommand::Rm(t) => {
            store.remove(t.key).expect("err rm");
        }
    }
}
