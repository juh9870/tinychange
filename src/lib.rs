use crate::config::CommandOpts;
use clap::{Parser, Subcommand};
use commands::merge::MergeArgs;
use commands::new::NewArgs;
use miette::{Context, Diagnostic, IntoDiagnostic};
use std::path::PathBuf;
use thiserror::Error;

mod commands;
mod config;
mod naming;
mod tinychange;

#[cfg(test)]
mod test;

#[derive(Debug, Parser)]
#[command(
    version,
    propagate_version = true,
    about = "A tool for creating tiny changelogs on a fly!"
)]
pub struct TinyChangeArgs {
    /// Disable all interactive prompts
    #[arg(short = 'I', long)]
    non_interactive: bool,
    #[arg(short, long)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<TinyChangeSubcommand>,
}

#[derive(Debug, Clone, Subcommand)]
enum TinyChangeSubcommand {
    /// Create a new tinychange file
    New(NewArgs),
    /// Merge all tinychanges into the changelog
    Merge(MergeArgs),
    /// Initialize tinychange configuration in the project
    Init,
}

#[derive(Debug, Error, Diagnostic)]
#[error("Error: {error}")]
struct ErrorHelp {
    #[diagnostic]
    error: miette::Error,
    #[help]
    help: String,
}

pub fn run(args: TinyChangeArgs, command_name: &str) -> miette::Result<()> {
    let command = args
        .command
        .unwrap_or_else(|| TinyChangeSubcommand::New(Default::default()));

    let config_path = args
        .config
        .unwrap_or_else(|| PathBuf::from("tinychange.toml"));

    if matches!(command, TinyChangeSubcommand::Init) {
        return commands::init::run(config_path, command_name);
    }

    let config = fs_err::read_to_string(&config_path)
        .into_diagnostic()
        .map_err(|e| ErrorHelp {
            error: e,
            help: format!(
                "Run `{} init` to initialize tinychange in your project",
                command_name
            ),
        })?;

    let config = toml::from_str(&config)
        .into_diagnostic()
        .context("Failed to read configuration file")?;

    let workdir = std::env::current_dir().into_diagnostic()?;

    let config_folder = config_path
        .parent()
        .map(|p| p.to_owned())
        .unwrap_or_else(|| workdir.clone());

    let opts = CommandOpts::new(
        args.non_interactive,
        !args.non_interactive,
        config_folder,
        workdir,
        command_name.to_owned(),
        config,
    )?;

    match command {
        TinyChangeSubcommand::New(cmd) => cmd.run(opts),
        TinyChangeSubcommand::Merge(cmd) => cmd.run(opts),
        TinyChangeSubcommand::Init => unreachable!("Handled above"),
    }
}
