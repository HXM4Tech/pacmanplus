use crate::config::Config;

use crate::exec;
use crate::printtr;
use crate::util::ask;

use std::fs::remove_dir_all;

use anyhow::{Context, Result};
use tr::tr;

pub fn clean(config: &Config) -> Result<()> {
    if config.mode.repo() {
        exec::pacman(config, &config.args)?;
    }

    if config.mode.aur() {
        if config.mode.repo() {
            println!();
        }

        printtr!("Cache directory: {}", config.cache_dir.display());
        let question = tr!("Do you want to clean ALL AUR packages from cache?");

        if ask(config, &question, true) && config.cache_dir.exists() {
            printtr!("removing AUR source files from cache directory...");

            for entry in config.cache_dir.read_dir().with_context(|| {
                tr!(
                    "could not read cache directory '{}'",
                    config.cache_dir.display().to_string()
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
