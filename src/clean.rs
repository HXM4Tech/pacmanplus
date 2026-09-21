use crate::config::Config;

use crate::exec;
use crate::printtr;
use crate::util::ask;

use std::fs::remove_dir_all;

use anyhow::{Context, Result};
use tr::tr;

pub fn clean_pacman_download_dirs() -> Result<()> {
    let download_dirs = std::fs::read_dir("/var/cache/pacman/pkg/")
        .with_context(|| tr!("could not read pacman cache directory"))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_str()?;
            if file_name_str.starts_with("download-") {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for dir in download_dirs {
        let _ = remove_dir_all(&dir);
    }

    Ok(())
}

pub fn clean(config: &Config) -> Result<()> {
    clean_pacman_download_dirs()?;

    if config.mode.repo() {
        exec::pacman(config, &config.args)?;
    }

    if config.mode.aur() {
        if config.mode.repo() {
            println!();
        }

        let cache_dir = config.cache_dir.display().to_string() + "/";

        printtr!("Cache directory: {}", cache_dir);
        let question = tr!("Do you want to clean ALL AUR packages from cache?");

        if ask(config, &question, true) && config.cache_dir.exists() {
            printtr!("removing AUR source files from cache directory...");

            for entry in config.cache_dir.read_dir().with_context(|| {
                tr!(
                    "could not read cache directory '{}'",
                    cache_dir
                )
            })? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let _ = remove_dir_all(&path);
                }
            }
        }
    }
    Ok(())
}
