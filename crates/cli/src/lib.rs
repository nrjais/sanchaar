use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};

pub fn main() {
    let matches = command!()
        .arg(
            arg!(-p --path <PATH> "Collection directory path")
                .required(false)
                .default_value("./")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-e --env <NAME> "Environment name")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .subcommand(
            Command::new("run").about("Run a request by path").arg(
                arg!(<path> "Path to request file")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
            ),
        )
        .get_matches();

    // let path = matches.value_of("path").map(|p| p.to_string());
}
