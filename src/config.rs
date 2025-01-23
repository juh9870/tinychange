use crate::naming::NameType;
use miette::bail;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub tinylogs_dir: PathBuf,
    pub changelog: PathBuf,

    pub categories: Vec<String>,
    #[serde(default)]
    pub naming: NameType,
    #[serde(default)]
    pub max_filename_length: Option<usize>,
}

#[derive(Debug)]
pub struct CommandOpts {
    silent: bool,
    interactive: bool,
    tinychanges_dir: PathBuf,
    changelog: PathBuf,
    workdir: PathBuf,
    command_name: String,
    config: Config,
}

impl CommandOpts {
    pub fn new(
        silent: bool,
        interactive: bool,
        config_dir: PathBuf,
        workdir: PathBuf,
        command_name: String,
        config: Config,
    ) -> miette::Result<Self> {
        let tinylogs_dir = config_dir.join(&config.tinylogs_dir);
        let changelog = config_dir.join(&config.changelog);

        if !tinylogs_dir.starts_with(&config_dir) {
            bail!("Tinylogs directory is outside of the project directory");
        }

        if !changelog.starts_with(&config_dir) {
            bail!("Changelog file is outside of the project directory");
        }

        Ok(Self {
            silent,
            interactive,
            tinychanges_dir: tinylogs_dir,
            changelog,
            workdir,
            command_name,
            config,
        })
    }
}

impl CommandOpts {
    pub fn println(&self, message: &str) {
        if !self.silent {
            println!("{}", message);
        }
    }

    pub fn interactive(&self) -> bool {
        self.interactive
    }

    pub fn categories(&self) -> &[String] {
        &self.config.categories
    }

    pub fn workdir(&self) -> &Path {
        self.workdir.as_path()
    }

    pub fn tinychanges_dir(&self) -> &Path {
        self.tinychanges_dir.as_path()
    }

    pub fn changelog_file(&self) -> &Path {
        self.changelog.as_path()
    }

    pub fn naming(&self) -> &NameType {
        &self.config.naming
    }

    pub fn max_filename_length(&self) -> Option<usize> {
        self.config.max_filename_length
    }

    pub fn command_name(&self) -> &str {
        &self.command_name
    }
}
