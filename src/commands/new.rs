use crate::config::CommandOpts;
use crate::tinychange::TinyChange;
use clap::Args;
use miette::{bail, Context, IntoDiagnostic};
use std::path::Path;

#[derive(Debug, Default, Clone, Args)]
pub struct NewArgs {
    #[arg(short, long)]
    kind: Option<String>,
    #[arg(short, long)]
    message: Option<String>,
    #[arg(short, long)]
    author: Option<String>,
}

impl NewArgs {
    pub fn run(self, opts: CommandOpts) -> miette::Result<()> {
        let author = if let Some(author) = self.author {
            author
        } else if let Some(name) = find_author(opts.workdir())? {
            opts.println(&format!("Found author from git config: {}", name));
            name
        } else if opts.interactive() {
            inquire::Text::new("Who is the author of this change?")
                .prompt()
                .into_diagnostic()?
        } else {
            bail!("No author provided")
        };

        let kind = if let Some(kind) = self.kind {
            if opts.categories().contains(&kind) {
                kind
            } else {
                bail!("Unknown change type: {}", kind)
            }
        } else if opts.interactive() {
            inquire::Select::new("What kind of change is this?", opts.categories().to_vec())
                .prompt()
                .into_diagnostic()?
        } else {
            bail!("No change type provided")
        };

        let message = if let Some(message) = self.message {
            message
        } else if opts.interactive() {
            inquire::Text::new("What is the message for this change?")
                .prompt()
                .into_diagnostic()?
        } else {
            bail!("No message provided")
        };

        if message.is_empty() {
            bail!("Empty message")
        }

        let change = TinyChange {
            kind,
            message,
            author,
        };

        let name = change.filename(&opts)?;
        let path = opts.tinychanges_dir().join(&name);

        fs_err::create_dir_all(opts.tinychanges_dir())
            .into_diagnostic()
            .context("Failed to create tinychange directory")?;
        fs_err::write(path, change.serialize())
            .into_diagnostic()
            .context("Failed to write tinychange file")?;

        Ok(())
    }
}

fn find_author(workdir: &Path) -> miette::Result<Option<String>> {
    if let Ok(repo) = gix::discover(workdir) {
        if let Some(author) = repo.author().transpose().into_diagnostic()? {
            return Ok(Some(author.name.to_string()));
        }
    }

    Ok(None)
}
