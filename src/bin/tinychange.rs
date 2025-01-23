use clap::Parser;
use tinychange::TinyChangeArgs;

fn main() -> miette::Result<()> {
    tinychange::run(TinyChangeArgs::parse(), "tinychange")
}
