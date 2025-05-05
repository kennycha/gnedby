use super::commands::Args;
use anyhow::{Context, Result};
use clap::Parser;

pub fn parse_args() -> Result<Args> {
    Args::try_parse().context("Failed to parse command line arguments")
}
