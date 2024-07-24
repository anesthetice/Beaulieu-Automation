use clap::{
    Arg, ArgAction, Command, command, ArgMatches
};
use new::new_subcommand;
use run::run_subcommand;
use self::ProcessOutput as PO;

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