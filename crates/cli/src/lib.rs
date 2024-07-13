pub mod run;
pub mod test;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use clap::{arg, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name = "sanchaar")]
struct Cli {
    /// Path to collection, defaults to current directory
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    path: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run a request file
    #[command(arg_required_else_help = true)]
    Run {
        /// Path to request file
        request: PathBuf,

        /// Run in verbose mode
        /// If not provided, only body is printed
        /// If provided, status, headers, duration, and size are also printed
        #[arg(short, long)]
        verbose: bool,
    },
    /// Run a request file
    Test {
        /// Path to test specific file or directory
        /// If not provided, all tests are run
        #[arg(value_name = "PATH")]
        path: Option<PathBuf>,
    },
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { request, verbose } => run::run(cli.path, request, verbose).await,
        Commands::Test { path } => test::test(cli.path, path.unwrap_or_default()).await,
    }
}
