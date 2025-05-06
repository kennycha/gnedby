use super::commands::Args;
use anyhow::Result;
use clap::Parser;
use std::process;

pub fn parse_args() -> Result<Args> {
    match Args::try_parse() {
        Ok(args) => Ok(args),
        Err(err) => {
            if err.kind() == clap::error::ErrorKind::DisplayHelp {
                err.print().expect("Failed to print help information");
                process::exit(0);
            }
            Err(anyhow::anyhow!("Failed to parse command line arguments"))
        }
    }
}
