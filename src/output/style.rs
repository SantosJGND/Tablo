use crate::formatter::{display_width, truncate_to_width};
use crate::table::{Cell, CellKind};
use owo_colors::{AnsiColors, Style};

/// Which styling is active for one render. `color: false` makes every method
/// return its input unchanged, so plain output is byte-identical.
#[derive(Debug, Clone, Copy, Default)]
pub struct Palette {
    pub color: bool,
    pub zebra: bool,
    pub negative_highlight: bool,
}

const ZEBRA_BG: AnsiColors = AnsiColors::BrightBlack;

impl Palette {
    /// Header text: bold.
    pub fn header(&self, text: &str) -> String {
        self.paint(text, Style::new().bold())
    }

    /// Cell text styled by its kind. `zebra_row` adds the subtle background.
    pub fn cell(&self, kind: CellKind, text: &str, zebra_row: bool) -> String {
        let mut style = Style::new();
        style = match kind {
            CellKind::Text => style,
            CellKind::Number { negative: true } if self.negative_highlight => {
                style.color(AnsiColors::Yellow)
            }
            CellKind::Number { .. } => style.color(AnsiColors::Cyan),
            CellKind::Boolean(true) => style.color(AnsiColors::Green),
            CellKind::Boolean(false) => style.color(AnsiColors::Red),
            CellKind::Null => style.dimmed(),
        };
        if zebra_row && self.zebra {
            style = style.on_color(ZEBRA_BG);
        }
        self.paint(text, style)
    }

    /// Padding and separators; carries only the zebra background.
    pub fn filler(&self, text: &str, zebra_row: bool) -> String {
        if zebra_row && self.zebra {
            self.paint(text, Style::new().on_color(ZEBRA_BG))
        } else {
            text.to_string()
        }
    }

    fn paint(&self, text: &str, style: Style) -> String {
        if !self.color || text.is_empty() {
            return text.to_string();
        }
        // Styled<T> resets attributes at the end of each span, so styles do
        // not leak into following output.
        style.style(text).to_string()
    }
}

/// Truncates by display width first, then styles the visible text and pads
/// with unstyled-width spaces. ANSI codes are never measured.
pub fn fit_cell(palette: &Palette, cell: &Cell, width: usize, zebra_row: bool) -> String {
    let truncated = truncate_to_width(&cell.text, width);
    let pad = width.saturating_sub(display_width(&truncated));
    format!(
        "{}{}",
        palette.cell(cell.kind, &truncated, zebra_row),
        palette.filler(&" ".repeat(pad), zebra_row)
    )
}

pub fn fit_header(palette: &Palette, text: &str, width: usize) -> String {
    let truncated = truncate_to_width(text, width);
    let pad = width.saturating_sub(display_width(&truncated));
    format!("{}{}", palette.header(&truncated), " ".repeat(pad))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::{grid, plain};
    use crate::table::Table;

    const ON: Palette = Palette {
        color: true,
        zebra: true,
        negative_highlight: true,
    };

    /// Removes SGR sequences (ESC [ ... m) so visible content can be compared.
    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\u{1b}' {
                for next in chars.by_ref() {
                    if next == 'm' {
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    #[test]
    fn disabled_palette_emits_no_escape_sequences() {
        let p = Palette::default();
        assert_eq!(p.header("name"), "name");
        assert_eq!(p.cell(CellKind::Boolean(true), "true", true), "true");
        assert_eq!(
            p.cell(CellKind::Number { negative: true }, "-1", true),
            "-1"
        );
        assert_eq!(p.filler("  ", true), "  ");
    }

    #[test]
    fn header_is_bold_and_reset() {
        let s = ON.header("name");
        assert!(s.starts_with("\u{1b}["), "{s:?}");
        assert!(s.contains('1'), "bold code expected: {s:?}");
        assert!(s.ends_with("\u{1b}[0m"), "reset expected: {s:?}");
        assert_eq!(strip_ansi(&s), "name");
    }

    #[test]
    fn boolean_colours_are_distinct() {
        let t = ON.cell(CellKind::Boolean(true), "true", false);
        let f = ON.cell(CellKind::Boolean(false), "false", false);
        assert!(t.contains("32"), "green expected: {t:?}");
        assert!(f.contains("31"), "red expected: {f:?}");
        assert_eq!(strip_ansi(&t), "true");
        assert_eq!(strip_ansi(&f), "false");
    }

    #[test]
    fn null_is_dimmed_and_empty_null_emits_no_codes() {
        let s = ON.cell(CellKind::Null, "null", false);
        assert!(s.contains('2'), "dim expected: {s:?}");
        assert_eq!(ON.cell(CellKind::Null, "", false), "");
    }

    #[test]
    fn negative_numbers_highlight_only_when_enabled() {
        let plain_neg = Palette {
            negative_highlight: false,
            ..ON
        };
        let hi = ON.cell(CellKind::Number { negative: true }, "-5", false);
        let lo = plain_neg.cell(CellKind::Number { negative: true }, "-5", false);
        assert!(hi.contains("33"), "yellow expected: {hi:?}");
        assert!(lo.contains("36"), "cyan expected: {lo:?}");
    }

    #[test]
    fn zebra_background_only_on_zebra_rows() {
        let off = ON.cell(CellKind::Text, "x", false);
        let on = ON.cell(CellKind::Text, "x", true);
        assert!(!off.contains("100"), "{off:?}");
        assert!(on.contains("100"), "{on:?}");
    }

    #[test]
    fn fit_cell_width_uses_unstyled_text() {
        let cell = Cell {
            text: "José".to_string(),
            kind: CellKind::Boolean(true),
        };
        let fitted = fit_cell(&ON, &cell, 8, true);
        assert_eq!(display_width(&strip_ansi(&fitted)), 8);
        assert_eq!(strip_ansi(&fitted), "José    ");
    }

    #[test]
    fn fit_cell_truncates_before_styling() {
        let cell = Cell::text("abcdefghij");
        let fitted = fit_cell(&ON, &cell, 6, false);
        assert_eq!(strip_ansi(&fitted), "abc...");
    }

    #[test]
    fn fit_header_pads_without_counting_codes() {
        let fitted = fit_header(&ON, "id", 5);
        assert_eq!(display_width(&strip_ansi(&fitted)), 5);
    }

    const OFF: Palette = Palette {
        color: false,
        zebra: false,
        negative_highlight: false,
    };

    fn sample_table() -> Table {
        let typed = |text: &str, kind| Cell {
            text: text.to_string(),
            kind,
        };
        Table {
            headers: Some(vec!["名前".into(), "ok".into(), "n".into()]),
            rows: vec![
                vec![
                    Cell::text("José"),
                    typed("true", CellKind::Boolean(true)),
                    typed("-3", CellKind::Number { negative: true }),
                ],
                vec![
                    Cell::text("李雷"),
                    typed("false", CellKind::Boolean(false)),
                    typed("4", CellKind::Number { negative: false }),
                ],
                vec![
                    typed("", CellKind::Null),
                    typed("true", CellKind::Boolean(true)),
                    Cell::text("x"),
                ],
            ],
        }
    }

    #[test]
    fn plain_alignment_is_independent_of_ansi() {
        let table = sample_table();
        let on = plain::render(&table, &ON);
        let off = plain::render(&table, &OFF);
        assert!(on.contains("\u{1b}["));
        assert_eq!(strip_ansi(&on), off);
    }

    #[test]
    fn grid_alignment_is_independent_of_ansi() {
        let table = sample_table();
        let on = grid::render(&table, &ON);
        let off = grid::render(&table, &OFF);
        assert!(on.contains("\u{1b}["));
        assert_eq!(strip_ansi(&on), off);
    }

    #[test]
    fn zebra_shades_only_odd_data_rows() {
        let out = plain::render(&sample_table(), &ON);
        let lines: Vec<&str> = out.lines().collect();
        // lines[0] is the header; data rows are lines[1..].
        assert!(!lines[1].contains("100m"), "{:?}", lines[1]);
        assert!(lines[2].contains("100m"), "{:?}", lines[2]);
        assert!(!lines[3].contains("100m"), "{:?}", lines[3]);
    }

    #[test]
    fn zebra_off_adds_no_background() {
        let no_zebra = Palette { zebra: false, ..ON };
        let out = plain::render(&sample_table(), &no_zebra);
        assert!(!out.contains("100m"));
    }
}
