use super::*;
use std::path::PathBuf;

pub(super) fn run_subcommand() -> Command {
    Command::new("run")
        .aliases(["start", "execute", "launch"])
        .about("Runs a BA application")
        .long_about("This subcommand is used to run a specified Beaulieu Automation applications\naliases: 'start', 'execute', 'launch'")
        .arg(Arg::new("path")
            .index(1)
            .required(true)
            .help("path of the application folder")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(Arg::new("repetitions")
            .required(false)
            .help("number of times to repeat script")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(usize))
        )
}

pub fn process_run_subcommand(arg_matches: &ArgMatches, resolution: (i32, i32)) -> anyhow::Result<PO>
{
    let Some(arg_matches) = arg_matches.subcommand_matches("run") else {
        return Ok(PO::Continue);
    };

    let Some(path) = arg_matches.get_one::<PathBuf>("path") else {
        tracing::error!("Failed to extract path from subcommand");
        return Ok(PO::Exit);
    };

    if !path.exists() {
        tracing::error!("Specified path does not exist");
        return Ok(PO::Exit)
    }

    // do stuff

    Ok(PO::Exit)
}