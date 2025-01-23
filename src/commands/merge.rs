use crate::config::CommandOpts;
use clap::Args;

#[derive(Debug, Default, Clone, Args)]
pub struct MergeArgs {}

impl MergeArgs {
    pub fn run(self, opts: CommandOpts) -> miette::Result<()> {
        todo!()
    }
}
