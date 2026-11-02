mod cli;
mod data;
mod formatter;
mod modes;
mod output;
mod source;
mod table;
mod terminal;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command, Style};
use std::io::{self, Write};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let output = match cli.command {
        Some(Command::Head {
            file,
            lines,
            delimiter,
            no_header,
            no_color: _,
        }) => modes::head::run(
            file.as_deref(),
            delimiter.as_deref(),
            lines,
            no_header,
            &Style::Plain,
        )?,
        Some(Command::Full {
            file,
            delimiter,
            no_header,
            style,
            no_color: _,
        }) => modes::full::run(file.as_deref(), delimiter.as_deref(), no_header, &style)?,
        None => modes::head::run(
            cli.file.as_deref(),
            cli.delimiter.as_deref(),
            cli.lines,
            cli.no_header,
            &cli.style,
        )?,
    };

    write_stdout(&output)
}

/// A closed pipe (e.g. `tablo f.csv | head`) is a normal end, not an error.
fn write_stdout(output: &str) -> Result<()> {
    let mut stdout = io::stdout().lock();
    match stdout
        .write_all(output.as_bytes())
        .and_then(|_| stdout.flush())
    {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(err) => Err(err.into()),
    }
}
