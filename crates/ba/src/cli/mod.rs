use self::ProcessOutput as PO;
use clap::{command, Arg, ArgAction, ArgMatches, Command};
use new::new_subcommand;
use run::run_subcommand;

mod new;
mod run;

pub fn cli() -> clap::Command {
    command!()
        .subcommand(new_subcommand())
        .subcommand(run_subcommand())
}

pub enum ProcessOutput {
    Continue,
    Exit,
}

pub use new::process_new_subcommand;
pub use run::process_run_subcommand;
