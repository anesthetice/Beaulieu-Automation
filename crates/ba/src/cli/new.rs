
use super::*;
use std::path::PathBuf;

pub(super) fn new_subcommand() -> Command {
    Command::new("new")
        .alias("create")
        .about("Create a new BA application")
        .long_about("This subcommand is used to create new Beaulieu Automation applications\naliases: 'create'")
        .arg(Arg::new("path")
                .index(1)
                .required(true)
                .help("path or name of the new application")
                .long_help("path or name of the new application\ne.g. 'new test' will create a new application named test in the terminal's current working directory")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(PathBuf))
            )
}

pub fn process_new_subcommand(arg_matches: &ArgMatches) -> anyhow::Result<PO>
{
    let Some(arg_matches) = arg_matches.subcommand_matches("new") else {
        return Ok(PO::Continue);
    };

    let Some(path) = arg_matches.get_one::<PathBuf>("path") else {
        tracing::error!("Failed to extract path from subcommand");
        return Ok(PO::Exit);
    };

    if path.exists() {
        tracing::error!("Specified path already exists");
        return Ok(PO::Exit)
    }

    // do stuff

    Ok(PO::Exit)
}