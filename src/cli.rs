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

    /// Input format (detected from file extension if not given)
    #[arg(long = "format", value_enum)]
    pub format: Option<Format>,

    /// Don't use colors
    #[arg(long = "no-color")]
    pub no_color: bool,

    /// Alternate background shading on data rows
    #[arg(long = "zebra")]
    pub zebra: bool,

    /// Highlight negative numbers
    #[arg(long = "negative-highlight")]
    pub negative_highlight: bool,

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
        /// Input format (detected from file extension if not given)
        #[arg(long = "format", value_enum)]
        format: Option<Format>,
        /// Don't use colors
        #[arg(long = "no-color")]
        no_color: bool,
        /// Alternate background shading on data rows
        #[arg(long = "zebra")]
        zebra: bool,
        /// Highlight negative numbers
        #[arg(long = "negative-highlight")]
        negative_highlight: bool,
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
        /// Input format (detected from file extension if not given)
        #[arg(long = "format", value_enum)]
        format: Option<Format>,
        /// Don't use colors
        #[arg(long = "no-color")]
        no_color: bool,
        /// Alternate background shading on data rows
        #[arg(long = "zebra")]
        zebra: bool,
        /// Highlight negative numbers
        #[arg(long = "negative-highlight")]
        negative_highlight: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Style {
    Plain,
    Grid,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Csv,
    Jsonl,
}

impl From<&Format> for crate::source::Format {
    fn from(format: &Format) -> Self {
        match format {
            Format::Csv => crate::source::Format::Csv,
            Format::Jsonl => crate::source::Format::Jsonl,
        }
    }
}
