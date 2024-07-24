use anyhow::Context;

use crate::{keymap::DEFAULT_KEYMAP, mousemap::DEFAULT_MOUSEMAP};

use super::*;
use std::{fs::OpenOptions, io::Write, path::{absolute, PathBuf}};

pub(super) fn new_subcommand() -> Command {
    Command::new("new")
        .alias("create")
        .about("Create a new BA application")
        .long_about("This subcommand is used to create new 'Beaulieu Automation' applications\naliases: 'create'")
        .arg(Arg::new("path")
                .index(1)
                .required(true)
                .help("path/name of the new application")
                .long_help("path/name of the new application\ne.g. 'new test' will create a new application named test in the terminal's current working directory")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(PathBuf))
            )
}

pub fn process_new_subcommand(arg_matches: &ArgMatches, resolution: (i32, i32)) -> anyhow::Result<PO>
{
    let Some(arg_matches) = arg_matches.subcommand_matches("new") else {
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

    if path.is_dir() {
        tracing::error!("Specified path/name already exists");
        return Ok(PO::Exit)
    } else if path.extension().is_some() {
        tracing::error!("Path required, got filepath instead");
        return Ok(PO::Exit)
    }

    tracing::info!("Attempting to create new application at absolute path '{}'", absolute_path.display());

    // create application folder
    std::fs::create_dir(&path).context(format!("Failed to create '{}'", absolute_path.display()))?;
    tracing::debug!("Created application folder");

    // define filepaths
    let main_filepath = &path.join("main.ba");
    let keymap_filepath = &path.join("keymap.json");
    let mousemap_filepath = &path.join("mousemap.json"); 
    let readme_filepath = &path.join("README.md"); 

    // generate main
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(&main_filepath)
        .context(format!("Failed to create main file with path '{}'", &main_filepath.display()))?
        .write_all(
            format!("// Resolution of the primary monitor for which this script was created\n// DO NOT MODIFY\ndefine RESOLUTION = {}, {}\n\n// The standard delay between actions given in milliseconds\ndefine DELAY_BETWEEN_ACTIONS = 50\n\n// The key used to stop the application if required\ndefine GLOBAL_HALT_KEY =  Esc", resolution.0, resolution.1)
                    .as_bytes()
        )?;
    tracing::debug!("Created main.ba");
    
    // generate readme
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(&readme_filepath)
        .context(format!("Failed to create readme file with path '{}'", &readme_filepath.display()))?
        .write_all(
        format!("## Title\n\n### description\n* describe what your application is used for here\n\n### layout\n* describe your desktop's initial layout for the application to function properly\n* please also include a screenshot of the entire initial desktop in the application folder")
                .as_bytes()
        )?;
    tracing::debug!("Created README.md");

    // generate keymap
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(&keymap_filepath)
        .context(format!("Failed to create keymap file with path '{}'", &keymap_filepath.display()))?
        .write_all(
            &serde_json::to_vec_pretty(&DEFAULT_KEYMAP[..]).unwrap()
        )?;
    tracing::debug!("Created keymap.json");
    
    // generate mousemap
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(&mousemap_filepath)
        .context(format!("Failed to create keymap file with path '{}'", &mousemap_filepath.display()))?
        .write_all(
            &serde_json::to_vec_pretty(&DEFAULT_MOUSEMAP[..]).unwrap()
        )?;    
    tracing::debug!("Created mousemap.json");
    
    Ok(PO::Exit)
}