use super::*;

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
            )
}

