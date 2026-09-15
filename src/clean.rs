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

        let question = tr!("Do you want to clean ALL AUR packages from cache?");

        printtr!("Clone Directory: {}", config.fetch.clone_dir.display());

        if ask(config, &question, true) && config.fetch.clone_dir.exists() {
            remove_dir_all(&config.fetch.clone_dir).with_context(|| {
                tr!(
                    "could not remove '{}'",
                    config.fetch.clone_dir.display().to_string()
                )
            })?;
        }

        if config.fetch.diff_dir.exists() {
            let _ = remove_dir_all(&config.fetch.diff_dir);
        }
    }
    Ok(())
}
