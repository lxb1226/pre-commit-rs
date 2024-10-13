use std::process::ExitCode;

use anstream::eprintln;
use clap::Parser;
use owo_colors::OwoColorize;

use crate::cli::{Cli, Command, ExitStatus};

mod cli;
mod config;
mod fs;
mod git;
mod hook;
mod identify;
mod store;

fn main() -> ExitCode {
    ctrlc::set_handler(move || {
        #[allow(clippy::exit, clippy::cast_possible_wrap)]
        std::process::exit(if cfg!(windows) {
            0xC000_013A_u32 as i32
        } else {
            130
        });
    })
    .expect("Error setting Ctrl-C handler");

    tracing_subscriber::fmt::init();

    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => err.exit(),
    };

    let result = match cli.command {
        Command::Install(options) => cli::install(
            cli.global_args.config,
            options.hook_type,
            options.install_hooks,
        ),
        Command::Run(options) => {
            cli::run(cli.global_args.config, options.hook_id, options.hook_stage)
        }
        _ => {
            eprintln!("Command not implemented yet");
            Ok(ExitStatus::Failure)
        }
    };

    match result {
        Ok(code) => code.into(),
        Err(err) => {
            let mut causes = err.chain();
            eprintln!("{}: {}", "error".red().bold(), causes.next().unwrap());
            for err in causes {
                eprintln!("  {}: {}", "caused by".red().bold(), err);
            }
            ExitStatus::Error.into()
        }
    }
}
