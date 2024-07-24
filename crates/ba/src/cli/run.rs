use anyhow::Context;

use super::*;
use std::{fs::OpenOptions, io::Read, path::{absolute, PathBuf}};

pub(super) fn run_subcommand() -> Command {
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
            .short('r')
            .short_alias('n')
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
        tracing::error!("Failed to extract a valid path/name");
        return Ok(PO::Exit);
    };
    let absolute_path = absolute(&path)
    .map_err(|err| {
        tracing::warn!("Failed to absolutize given path '{}' due to '{}'", path.display(), err);
        err
    })
    .unwrap_or(path.clone());

    if !path.is_dir() {
        tracing::error!("Specified path/name does not exist");
        return Ok(PO::Exit)
    } else if path.extension().is_some() {
        tracing::error!("Path required, got filepath instead");
        return Ok(PO::Exit)
    }

    // filepaths
    let main_filepath = &absolute_path.join("main.ba");
    let keymap_filepath = &absolute_path.join("keymap.json");
    let mousemap_filepath = &absolute_path.join("mousemap.json"); 

    // load KeyMap
    crate::keymap::KeyMap::init(keymap_filepath)?;
    tracing::debug!("KeyMap initialized");

    // load MouseMap
    crate::mousemap::MouseMap::init(mousemap_filepath)?;
    tracing::debug!("MouseMap initialized");

    // load main file
    let mut input: String = String::new();
    std::fs::OpenOptions::new()
        .read(true)
        .open(&main_filepath)
        .context(format!("Failed to read/open file with path '{}'", main_filepath.display()))?
        .read_to_string(&mut input)?;

    let mut parser = crate::compiler::Parser::new(&input);
    let parsed = parser.process()?;
    println!("{:#?}", parsed);

    Ok(PO::Exit)
}