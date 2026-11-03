# Contributing to tablo

Thanks for helping improve `tablo`. This document covers the development setup,
checks, and the expectations for changes.

## Development environment

- Rust 1.84 or newer. The package declares `rust-version = "1.84"`.
- A Cargo toolchain with `rustfmt` and `clippy` installed, for example through
  `rustup component add rustfmt clippy`.

## Build and test

```bash
cargo build
cargo test --all-targets
```

Release build:

```bash
cargo build --release
```

The integration tests run the built binary and use fixtures in
`tests/fixtures/`. They cover CSV, JSONL, colour, terminal redirection, Unicode
alignment, malformed input, and exit status.

## Formatting and linting

Run all of these before submitting a change:

```bash
cargo fmt --all -- --check
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets
```

Do not suppress warnings or weaken checks to make a change pass. If a lint is a
genuine false positive, explain why in the pull request.

## Project structure

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the design and module map.
In short:

- `src/cli.rs` defines the command-line surface.
- `src/source/` contains input adapters behind the `DataSource` trait.
- `src/data/` contains the shared schema and value model.
- `src/modes/` coordinates opening, row collection, table assembly, and
  rendering.
- `src/output/` contains layouts and styling.
- `src/table.rs` and `src/formatter.rs` hold render-ready types and width
  helpers.

## Bug reports

Include:

- The `tablo` version or commit.
- The command you ran, including flags.
- Whether input came from a file or stdin.
- A small, non-sensitive input sample that reproduces the problem.
- The observed output and the expected output.
- Your terminal or, for redirected output, the destination program.

Do not include private data, credentials, or personal information in reports or
fixtures.

## Pull requests

- Keep each pull request focused on one change.
- Describe the user-visible effect and any behaviour changes.
- Add or update tests for the changed behaviour.
- Update documentation when flags, defaults, or output change.
- Keep formatting and lint checks clean.
- Do not commit secrets, local paths, build artifacts, or editor files.

## Adding an input format

1. Add a module under `src/source/` that implements `DataSource`.
2. Add a variant to `source::Format` and to `cli::Format`.
3. Construct the adapter in `open_source` and extend extension detection in
   `format_from_extension`.
4. Produce `Schema` and `Record` values; keep format-specific inference in the
   adapter.
5. Add fixtures and integration tests.

Formats that require reading metadata before streaming should still fit behind
the same trait. Report capabilities honestly through `SourceCapabilities`.

## Adding rendering functionality

- Keep layout and styling separate. Compute widths from unstyled text.
- Apply colour only through `Palette`, and only during final output.
- Reset styles locally so they do not leak.
- Add a renderer module under `src/output/` and dispatch it from
  `modes::render` if it is a new style.
- Make sure redirected output remains free of ANSI sequences.

## Testing expectations

- Add tests for new behaviour and for bug fixes.
- Cover both colour-enabled and colour-disabled paths where styling is involved.
- Include a Unicode case when changing width, padding, or truncation.
- Check redirected output when changing anything that may emit colour.
- Update fixtures only when the data is required by a test; keep them small and
  free of personal information.
