use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "tablo")]
#[command(about = "Inspect tabular data in the terminal", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Input file (reads from stdin if not provided)
    pub file: Option<String>,

    /// Explicit delimiter
    #[arg(short = 'd', long = "delimiter")]
    pub delimiter: Option<String>,

    /// Number of rows to show in head mode
    #[arg(short = 'n', long = "lines", default_value = "10")]
    pub lines: usize,

    /// Treat first row as data, not header
    #[arg(long = "no-header")]
    pub no_header: bool,

    /// Output style
    #[arg(long = "style", value_enum, default_value = "plain")]
    pub style: Style,

    /// Don't use colors
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// Show row numbers
    #[arg(long = "row-numbers")]
    pub row_numbers: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Show first N rows (default)
    Head {
        /// Input file (reads from stdin if not provided)
        file: Option<String>,
        /// Number of rows to show
        #[arg(short = 'n', long = "lines", default_value = "10")]
        lines: usize,
        /// Explicit delimiter
        #[arg(short = 'd', long = "delimiter")]
        delimiter: Option<String>,
        /// Treat first row as data, not header
        #[arg(long = "no-header")]
        no_header: bool,
        /// Don't use colors
        #[arg(long = "no-color")]
        no_color: bool,
    },
    /// Show all rows
    Full {
        /// Input file (reads from stdin if not provided)
        file: Option<String>,
        /// Explicit delimiter
        #[arg(short = 'd', long = "delimiter")]
        delimiter: Option<String>,
        /// Treat first row as data, not header
        #[arg(long = "no-header")]
        no_header: bool,
        /// Output style
        #[arg(long = "style", value_enum, default_value = "plain")]
        style: Style,
        /// Don't use colors
        #[arg(long = "no-color")]
        no_color: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Style {
    Plain,
    Grid,
}
