# tablo

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust 1.84+](https://img.shields.io/badge/rust-1.84%2B-orange.svg)](Cargo.toml)

Inspect tabular data in the terminal.

`tablo` is a small command-line tool for looking at CSV, TSV, and JSONL data
without leaving the shell. It streams input, aligns columns by display width
(including non-ASCII text), and can add restrained colour so values are easier
to scan. Output is plain by default and is safe to redirect or pipe.

## Features

- CSV, TSV, and JSONL input from a file or stdin.
- Delimiter detection for CSV (comma, semicolon, tab, or pipe).
- Format detection from `.jsonl` / `.ndjson` file extensions.
- `head` preview mode (default) and `full` mode.
- Plain and boxed `grid` layouts.
- Unicode-aware column alignment and truncation.
- Optional colour: bold headers, semantic value colours, zebra rows, and
  negative-number highlighting.
- Colour only when stdout is a terminal; respects `NO_COLOR` and `--no-color`.
- JSONL objects with nested objects flattened to dotted column names.
- Dimension footer in `full` mode and a `...` marker when preview rows are
  truncated.

## Installation

### From source

Building requires Rust 1.84 or newer.

```bash
git clone https://github.com/SantosJGND/Tablo.git
cd Tablo
cargo build --release
./target/release/tablo --help
```

Or install the binary into your Cargo bin directory:

```bash
cargo install --path .
```

### From crates.io

`tablo` is not published on crates.io yet. The crate name `tablo` is currently
used by an unrelated project, so do not use `cargo install tablo` for this
tool. Install from source as shown above until publication is resolved.

## Quick start

With the sample file in this repository:

```bash
tablo data.csv
```

```text
name     age  city
Alice    31   London
Bob      25   Paris
Charlie  42   Berlin
...
```

Show all rows with a boxed layout:

```bash
tablo full data.csv --style grid
```

```text
┌─────────┬─────┬────────┐
│ name    │ age │ city   │
├─────────┼─────┼────────┤
│ Alice   │ 31  │ London │
│ Bob     │ 25  │ Paris  │
│ Charlie │ 42  │ Berlin │
└─────────┴─────┴────────┘
3 rows × 3 columns
```

Read from stdin:

```bash
cat data.csv | tablo
```

Limit a preview to five rows:

```bash
tablo head data.csv -n 5
```

Disable colour explicitly:

```bash
tablo data.csv --no-color
```

Inspect JSONL:

```bash
tablo full data.jsonl
```

Use an explicit delimiter when sniffing is not appropriate:

```bash
tablo full data.csv -d ';'
```

## Input formats

CSV and TSV are read with the same parser. When `--delimiter` is not given,
`tablo` samples the first 8 KiB and chooses among comma, semicolon, tab, and
pipe, falling back to comma. The first row is treated as a header unless
`--no-header` is passed.

JSONL (also accepted with an `.ndjson` extension) treats each non-blank line as
a JSON object. Keys become columns, nested objects are flattened with dotted
names, and arrays or empty objects are shown as compact JSON text. The schema
is inferred from the first 1000 non-blank records; keys that first appear after
that sample are not displayed.

File extensions select the format: `.jsonl` and `.ndjson` mean JSONL; every
other extension means CSV. Stdin defaults to CSV. Use `--format csv` or
`--format jsonl` to override detection.

## Commands and options

```text
Usage: tablo [OPTIONS] [FILE] [COMMAND]

Commands:
  head  Show first N rows (default)
  full  Show all rows
```

Top-level options apply to the default `head` mode:

| Option | Description |
| --- | --- |
| `-d, --delimiter <DELIMITER>` | Explicit delimiter. Use a single character, `tab`, or `\t`. |
| `-n, --lines <LINES>` | Number of preview rows. Default: `10`. |
| `--no-header` | Treat the first CSV row as data and name columns `column_1`, `column_2`, ... |
| `--style <STYLE>` | `plain` (default) or `grid`. |
| `--format <FORMAT>` | `csv` or `jsonl`. |
| `--no-color` | Never emit ANSI colour. |
| `--zebra` | Shade alternate data rows when colour is enabled. |
| `--negative-highlight` | Colour negative numbers differently from other numbers. |

`tablo head` accepts the same preview options except `--style`; it always uses
the plain layout. `tablo full` accepts the same options except `--lines`; it
always reads every row.

The `full` command appends a dimension footer such as `3 rows × 3 columns`.
The default `head` command prints `...` on its own line when more rows exist.

## Rendering and colour

`plain` separates columns with two spaces. `grid` draws boxed borders and uses
the detected terminal width to shrink the widest columns when needed.

Colour is applied only when stdout is a terminal. It is disabled when stdout is
redirected, when `--no-color` is passed, or when the `NO_COLOR` environment
variable is set to a non-empty value. The palette is intentionally restrained:

- Headers: bold.
- Numbers: cyan.
- Negative numbers: yellow when `--negative-highlight` is enabled, otherwise
  cyan.
- Booleans: green for `true`, red for `false`.
- Null values and missing JSONL keys: empty cells.
- Zebra rows: a subtle bright-black background on every second data row when
  `--zebra` is enabled.

Column widths are calculated from unstyled text, so ANSI sequences never affect
alignment. Styles are reset locally to avoid leaking into later terminal
output.

## Exit behaviour

`tablo` exits with status `0` on success and status `1` on errors such as a
missing file, an invalid delimiter, or malformed JSONL. When output is piped
into a command that closes early (for example `tablo full big.csv | head`), the
broken pipe is treated as a normal end and no error is printed.

## Current limitations

- Parquet is not supported. There is no `schema` command and no column
  projection or selection option.
- Row numbers, sorting, filtering, and data transformations are not
  implemented.
- JSONL input must consist of one JSON object per non-blank line. Top-level
  arrays are rejected.
- JSONL columns are fixed after the first 1000 records.
- CSV type inference is display-preserving. Values that would change when
  reinterpreted, such as `00123`, `1.50`, `1e5`, `inf`, `NaN`, and `-0.0`,
  remain text.
- Delimiters are single-byte characters (plus the `tab` / `\t` aliases).
- Only Linux has been used for the checks in this release; other platforms
  have not been verified.

## Documentation

- [Usage guide](docs/USAGE.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Contributing](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

## License

MIT. See [LICENSE](LICENSE).

Repository: <https://github.com/SantosJGND/Tablo>  
Issues: <https://github.com/SantosJGND/Tablo/issues>
