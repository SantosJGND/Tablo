mod cli;
mod detect;
mod formatter;
mod input;
mod modes;
mod output;
mod parser;
mod table;
mod terminal;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command, Style};

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Determine mode
    match cli.command {
        Some(Command::Head { file, lines, delimiter, no_header, no_color: _ }) => {
            let output = modes::head::run(file.as_deref(), delimiter.as_deref(), lines, no_header)?;
            print!("{}", output);
        }
        Some(Command::Full { file, delimiter, no_header, style, no_color: _ }) => {
            let output = modes::full::run(file.as_deref(), delimiter.as_deref(), no_header, &style)?;
            print!("{}", output);
        }
        None => {
            // Default mode is head
            let file = cli.file.as_deref();
            let output = modes::head::run(file, cli.delimiter.as_deref(), cli.lines, cli.no_header)?;
            print!("{}", output);
        }
    }
    
    Ok(())
}
