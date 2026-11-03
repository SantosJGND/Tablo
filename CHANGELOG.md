# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-11-03

Initial public release.

### Added

- CSV and TSV input from a file or stdin.
- CSV delimiter detection for comma, semicolon, tab, and pipe, with an explicit
  `--delimiter` override.
- JSONL input from a file or stdin, including `.ndjson` extension detection.
- JSONL nested-object flattening with dotted column names.
- JSONL schema inference from the first 1000 non-blank records.
- Default `head` mode with a configurable row count and a `...` truncation
  marker.
- `full` mode with a row/column dimension footer.
- `plain` and `grid` rendering styles.
- Unicode display-width alignment and truncation.
- Terminal-width-aware column shrinking.
- Bold headers and semantic colours for numbers, booleans, and negative numbers.
- Optional zebra row shading.
- Colour enablement based on terminal detection, `NO_COLOR`, and `--no-color`.
- Integration tests covering CSV, JSONL, colour, redirection, Unicode, errors,
  and broken pipes.
- MIT license, README, usage guide, architecture notes, and contribution guide.

### Known limitations

- No Parquet support, `schema` command, column selection, row numbers, sorting,
  or filtering.
- JSONL columns are fixed after the first 1000 records.

[1.0.0]: https://github.com/SantosJGND/Tablo/releases/tag/v1.0.0
