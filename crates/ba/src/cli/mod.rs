use anyhow::anyhow;
use clap::{Arg, ArgAction, ArgMatches, Command, command};
use tracing::instrument;

mod new;
mod run;

pub fn cli(resolution: (i32, i32)) -> anyhow::Result<()> {
    let command = command!()
        .subcommand(new::subcommand())
        .subcommand(run::subcommand());

    let arg_matches = command.get_matches();

    match arg_matches.subcommand() {
        Some(("new", arg_matches)) => new::process(arg_matches, resolution),
        Some(("run", arg_matches)) => run::process(arg_matches, resolution),
        Some(_) => Ok(()),
        None => Ok(()),
    }
}
