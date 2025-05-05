mod cli;

use anyhow::Result;
use cli::parse_args;
use cli::Args;

fn main() -> Result<()> {
    if let Err(e) = run(parse_args()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn run(args: Args) -> Result<()> {
    println!("Command: {:?}", args.command);
    Ok(())
}
