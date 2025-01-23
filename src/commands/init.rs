use miette::{bail, Context, IntoDiagnostic};
use std::path::PathBuf;

pub fn run(config_path: PathBuf, command_name: &str) -> miette::Result<()> {
    if config_path.exists() {
        bail!("Configuration file already exists");
    }

    fs_err::write(&config_path, include_str!("../tinychange.default.toml"))
        .into_diagnostic()
        .context("Failed to write config file")?;

    println!("tinychange configuration initialized successfully! What's next?");
    println!("- Edit the configuration file at {}", config_path.display());
    println!("- Run `{}` to start creating tinychanges", command_name);

    Ok(())
}
