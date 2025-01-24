use crate::config::CommandOpts;
use crate::tinychange::TinyChange;
use clap::Args;
use miette::{bail, Context, IntoDiagnostic};
use std::ffi::OsStr;
use std::path::Path;

#[derive(Debug, Default, Clone, Args)]
pub struct NewArgs {
    /// The kind of change (must be one of the categories from the configuration)
    #[arg(short, long)]
    kind: Option<String>,
    /// The message describing the change
    #[arg(short, long)]
    message: Option<String>,
    /// The author of the change (defaults to the git author if not provided)
    #[arg(short, long)]
    author: Option<String>,
}

impl NewArgs {
    pub fn run(self, opts: CommandOpts) -> miette::Result<()> {
        let author = if let Some(author) = self.author {
            author
        } else if let Some(name) = find_author(&opts)? {
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
            let prompt = if kind.ends_with("ed") {
                format!("What got {}?", kind.to_lowercase())
            } else {
                "Describe the change".to_string()
            };
            inquire::Text::new(&prompt).prompt().into_diagnostic()?
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

fn find_author(opts: &CommandOpts) -> miette::Result<Option<String>> {
    fn run_cmd<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
        workdir: &Path,
        cmd: &str,
        args: I,
    ) -> Option<String> {
        let out = std::process::Command::new(cmd)
            .args(args)
            .current_dir(workdir)
            .output()
            .ok()?;

        if out.status.success() {
            String::from_utf8(out.stdout)
                .ok()
                .map(|x| x.trim().to_owned())
                .filter(|x| !x.is_empty())
        } else {
            None
        }
    }

    let name = if let Some(author) = run_cmd(opts.workdir(), "git", ["config", "author.name"]) {
        opts.println(&format!(
            "Found author from git author.name config: {}",
            author
        ));
        Some(author)
    } else if let Some(author) = std::env::var("GIT_AUTHOR_NAME")
        .ok()
        .map(|x| x.trim().to_owned())
        .filter(|x| !x.is_empty())
    {
        opts.println(&format!(
            "Found author from GIT_AUTHOR_NAME environment variable: {}",
            author
        ));
        Some(author)
    } else if let Some(author) = run_cmd(opts.workdir(), "git", ["config", "user.name"]) {
        opts.println(&format!(
            "Found author from git user.name config: {}",
            author
        ));
        Some(author)
    } else {
        None
    };

    Ok(name)
}
