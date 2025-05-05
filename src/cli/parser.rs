use super::commands::Args;
use clap::Parser;

pub fn parse_args() -> Args {
    Args::parse()
}
