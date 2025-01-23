use crate::config::CommandOpts;
use miette::{bail, miette};
use std::hash;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash)]
pub struct TinyChange {
    pub kind: String,
    pub message: String,
    pub author: String,
}

impl TinyChange {
    pub fn filename(&self, opts: &CommandOpts) -> miette::Result<String> {
        let hash = {
            let mut hasher = hash::DefaultHasher::new();
            self.hash(&mut hasher);
            hasher.finish()
        };

        Ok(format!(
            "{}.md",
            opts.naming().generate(hash, opts.max_filename_length())?
        ))
    }

    pub fn serialize(&self) -> String {
        format!(
            "- Author: {}\n- Kind: {}\n---\n{}",
            self.author, self.kind, self.message
        )
    }

    pub fn deserialize(opts: &CommandOpts, content: String) -> miette::Result<Self> {
        let mut lines = content.lines().peekable();

        let (field, author) = lines
            .next()
            .ok_or_else(|| miette!("Invalid format: Missing author field"))?
            .split_once(":")
            .ok_or_else(|| miette!("Invalid format: Malformed author field (missing colon)"))?;

        if field != "- Author" {
            bail!("Invalid format: Expected author field, got {}", field)
        }
        let author = author.trim().to_owned();

        while lines.peek().is_some_and(|line| line.is_empty()) {
            lines.next();
        }

        let (field, kind) = lines
            .next()
            .ok_or_else(|| miette!("Invalid format: Missing kind field"))?
            .split_once(":")
            .ok_or_else(|| miette!("Invalid format: Malformed kind field (missing colon)"))?;

        if field != "- Kind" {
            bail!("Invalid format: Expected kind field, got {}", field)
        }
        let kind = kind.trim().to_owned();

        while lines.peek().is_some_and(|line| line.is_empty()) {
            lines.next();
        }
        if lines.next() != Some("---") {
            bail!("Invalid format: missing message separator")
        }
        let message = lines.collect::<Vec<_>>().join("\n").trim().to_owned();

        if author.is_empty() {
            bail!("Empty author field")
        }

        if kind.is_empty() {
            bail!("Empty kind field")
        }

        if message.is_empty() {
            bail!("Empty message field")
        }

        if !opts.categories().contains(&kind) {
            bail!("Unknown change type: {}", kind)
        }

        Ok(Self {
            kind,
            message,
            author,
        })
    }
}
