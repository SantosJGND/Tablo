# tablo usage guide

This guide describes the behaviour implemented in `tablo` 1.0.0. Options or
formats that are not listed here are not supported.

## Command overview

```text
tablo [OPTIONS] [FILE] [COMMAND]

Commands:
  head  Show first N rows (default)
  full  Show all rows
```

When no subcommand is given, `tablo` behaves like `head`. The top-level options
belong to that default mode. The `head` and `full` subcommands each define their
own options; a top-level option placed after a subcommand is not accepted.

```bash
tablo data.csv                    # default head mode, top-level options
tablo head data.csv -n 5          # explicit head mode
tablo full data.csv --style grid  # full mode
```

`tablo head` does not accept `--style`; it always renders the plain layout.
`tablo full` does not accept `--lines`; it always reads every row. Both accept
`--delimiter`, `--no-header`, `--format`, `--no-color`, `--zebra`, and
`--negative-highlight`.

## Input formats and detection

### CSV and TSV

CSV and TSV share the CSV reader. The delimiter is chosen as follows:

1. `--delimiter` when supplied.
2. Otherwise, a sample of the first 8 KiB is scored for comma, semicolon, tab,
   and pipe. The best candidate is used, with comma as the fallback.

The first row is used as the header. Pass `--no-header` to treat it as data;
the generated column names are `column_1`, `column_2`, and so on.

Rows may be ragged. Every row and the header are padded to the widest row, so
missing trailing fields become empty cells and extra fields appear as additional
columns.

`--delimiter` accepts a single character, or `tab` / `\t` for a tab.

### JSONL

JSONL uses one JSON object per non-blank line. Blank lines are skipped. Each
line must be an object; a top-level array or scalar is rejected with an error.

Keys become columns. Nested objects are flattened using dots, for example:

```json
{"user":{"name":"Alice","address":{"city":"London"}}}
```

becomes the column `user.address.city`. Arrays, non-empty objects nested inside
arrays, and empty objects are rendered as compact JSON text.

The schema is inferred from the first 1000 non-blank records. Columns are
ordered by first appearance. Missing keys and JSON `null` values become empty
cells. A key that first appears after the sample is dropped from the output.

Column types are inferred across the sample:

- Integer and float mix to float.
- Any other type mix widens the column to text.
- Integers, floats, and booleans in a text column are converted to their text
  form for display.

JSON integers outside the signed 64-bit range are kept as text.

### Format selection

Format detection uses the file extension:

| Extension | Format |
| --- | --- |
| `.jsonl`, `.ndjson` | JSONL |
| anything else | CSV |

Stdin defaults to CSV. Override detection with `--format csv` or
`--format jsonl`.

```bash
tablo full records.ndjson
tablo full --format jsonl < records.log
tablo full data.jsonl --format csv
```

`--delimiter` does not apply to JSONL; combining the two is an error.

## Files and stdin

Pass a file path as the final positional argument, or omit it to read stdin.

```bash
tablo data.csv
tablo full data.csv
cat data.csv | tablo
cat data.jsonl | tablo --format jsonl
```

## Preview and full output

The default mode and the `head` subcommand show the first 10 rows:

```bash
tablo data.csv
tablo head data.csv
tablo head data.csv -n 25
```

If at least one more row exists beyond the preview, `tablo` prints `...` on its
own line. This is a row marker, not a cell-truncation marker.

The `full` subcommand reads and prints every row, followed by a dimension
footer:

```bash
tablo full data.csv
```

```text
...
3 rows × 3 columns
```

An empty CSV input produces no output in head mode. In full mode it produces
`0 rows × 0 columns`. A header-only CSV prints the header and
`0 rows × N columns`.

## Schema inspection and column selection

There is no `schema` command in this release. There is also no column selection
or projection option. `tablo` always displays all columns discovered in the
input.

Use `tablo full file.csv` to see the full set of columns for small inputs. For
JSONL, `head` and `full` display the columns inferred from the first 1000
records.

## Rendering styles

Two styles are available: `plain` and `grid`. `plain` is the default.

```bash
tablo full data.csv
tablo full data.csv --style grid
```

`plain` separates columns with two spaces. `grid` draws a box around the table
with a header separator.

The default head mode accepts a top-level `--style` option:

```bash
tablo data.csv --style grid
```

The `head` subcommand itself does not accept `--style`.

### Terminal width and truncation

Both layouts use the detected terminal width when one is available. If the
rendered table would be wider than the terminal, the widest columns are reduced
one character at a time until the table fits or no column can shrink below
three characters.

Cell text that does not fit is truncated with `...` (or with dots only when the
column is at most three characters wide). Width is measured with Unicode display
width, so combining characters and wide characters align correctly. Truncation
happens before styling, and padding uses the unstyled visible width.

When stdout is redirected, ANSI styling is disabled. Table layout is still
calculated normally; if no terminal width can be detected, no width constraint
is applied.

## Colour and terminal detection

Colour is enabled only when all of these are true:

- stdout is a terminal;
- `--no-color` is not present;
- `NO_COLOR` is unset or empty.

Consequently, redirected output never contains ANSI escape sequences, even when
`--zebra` or `--negative-highlight` is passed.

The palette is:

| Content | Style |
| --- | --- |
| Header | Bold |
| Text | Default foreground |
| Numbers | Cyan |
| Negative numbers | Yellow with `--negative-highlight`; otherwise cyan |
| `true` | Green |
| `false` | Red |
| Null / missing | Empty cell |
| Every second data row | Bright-black background with `--zebra` |

Zebra shading and negative highlighting require colour to be enabled. They have
no visible effect on redirected output.

Styles are reset at the end of each styled span, so they do not leak into
subsequent terminal output.

## Errors and troubleshooting

Errors are written to stderr and the process exits with status `1`.

| Situation | Message / behaviour |
| --- | --- |
| File cannot be opened | `Failed to read file: <path>` |
| Invalid delimiter | `Invalid delimiter: <value>` |
| Delimiter used with JSONL | `--delimiter does not apply to JSONL input` |
| Malformed JSONL | `line <n>, column <c>: invalid JSON (...)` |
| Non-object JSONL line | `line <n>: expected a JSON object` |
| Closed output pipe | Treated as success; no message |

Raw CSV parse errors are reported through the CSV reader. The first row is read
during source construction, so an invalid first row fails before rendering.

Common checks:

- Piped output has no colour: this is expected. Use a terminal or force styling
  only for development; `tablo` does not provide a flag to force colour on a
  non-terminal stdout.
- A JSONL column is missing: it may have first appeared after the 1000-record
  schema sample.
- A numeric CSV value is not coloured as a number: inference is conservative.
  Leading zeros, alternative decimal forms, exponents, `inf`, `NaN`, `+1`, and
  `-0.0` remain text.
- A column is truncated: the terminal width is being applied. Redirect the
  output or widen the terminal to see the full width.
- `--style` is rejected after `head`: pass it before the subcommand only for the
  default head mode, or use `full`.

## Representative examples

```bash
tablo data.csv
tablo data.csv -n 20
tablo data.csv --style grid
tablo data.csv --no-color
tablo data.csv --zebra --negative-highlight
tablo data.csv -d ';'
tablo head data.csv -n 5
tablo full data.csv
tablo full data.csv --style grid
cat data.csv | tablo
cat data.jsonl | tablo --format jsonl
tablo full records.ndjson
```

## Limitations

- No Parquet support.
- No schema subcommand.
- No column selection or projection.
- No row numbers, sorting, filtering, or transformations.
- JSONL schema is fixed after the first 1000 non-blank records.
- JSONL input is line-delimited objects only; a top-level JSON array is
  rejected.
- Single-byte delimiters only.
- Cross-platform behaviour has not been verified beyond Linux.
