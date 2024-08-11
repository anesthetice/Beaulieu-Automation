use anyhow::Context;

use crate::compiler::Engine;

use super::*;
use std::{
    io::Read,
    path::{absolute, PathBuf},
};

pub(super) fn subcommand() -> Command {
    Command::new("run")
        .aliases(["start", "execute", "launch"])
        .about("Runs the specified BA application")
        .long_about("This subcommand is used to run a specified 'Beaulieu Automation' application\naliases: 'start', 'execute', 'launch'")
        .arg(Arg::new("path")
            .index(1)
            .required(true)
            .help("path of the application folder")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(Arg::new("repetitions")
            .index(2)
            .required(false)
            .help("number of times to repeat the script")
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(usize))
        )
}

#[instrument(name = "run-subcommand", skip_all)]
pub(super) fn process(arg_matches: &ArgMatches, resolution: (i32, i32)) -> anyhow::Result<()> {
    let path = arg_matches
        .get_one::<PathBuf>("path")
        .ok_or(anyhow!("Failed to extract a valid path/name"))?;

    let absolute_path = absolute(path)
        .map_err(|err| {
            tracing::warn!(
                "Failed to absolutize given path '{}' due to '{}'",
                path.display(),
                err
            );
            err
        })
        .unwrap_or(path.clone());

    if !path.is_dir() {
        Err(anyhow!("Specified path/name does not exist"))?;
    } else if path.extension().is_some() {
        Err(anyhow!("Path required, got filepath instead"))?;
    }

    let nb_cycles = *arg_matches.get_one::<usize>("repetitions").unwrap_or(&1);

    // filepaths
    let main_filepath = absolute_path.join("main.ba");
    let keymap_filepath = absolute_path.join("keymap.json");
    let mousemap_filepath = absolute_path.join("mousemap.json");

    // load KeyMap
    crate::keymap::KeyMap::init(&keymap_filepath)?;
    tracing::debug!("KeyMap initialized");

    // load MouseMap
    crate::mousemap::MouseMap::init(&mousemap_filepath)?;
    tracing::debug!("MouseMap initialized");

    // load main file
    let mut input: String = String::new();
    std::fs::OpenOptions::new()
        .read(true)
        .open(&main_filepath)
        .context(format!(
            "Failed to read/open file with path '{}'",
            main_filepath.display()
        ))?
        .read_to_string(&mut input)?;

    let mut parser = crate::compiler::Parser::new(&input);
    let parsed = parser.process()?;

    let engine = Engine::new(parsed, resolution)?;
    engine.start(nb_cycles)?;

    Ok(())
}
