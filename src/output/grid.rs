use crate::formatter::display_width;
use crate::output::style::{fit_cell, fit_header, Palette};
use crate::table::{Cell, Table};
use crate::terminal::get_terminal_width;

pub fn render(table: &Table, palette: &Palette) -> String {
    let mut output = String::new();
    let term_width = get_terminal_width();

    let num_cols = match (&table.headers, table.rows.first()) {
        (Some(headers), _) => headers.len(),
        (None, Some(row)) => row.len(),
        (None, None) => return output,
    };
    let mut col_widths = vec![0usize; num_cols];

    if let Some(ref headers) = table.headers {
        for (i, cell) in headers.iter().enumerate().take(num_cols) {
            col_widths[i] = col_widths[i].max(display_width(cell));
        }
    }
    for row in &table.rows {
        for (i, cell) in row.iter().enumerate().take(num_cols) {
            col_widths[i] = col_widths[i].max(display_width(&cell.text));
        }
    }

    // Apply terminal width constraints
    if let Some(tw) = term_width {
        let total_width: usize = col_widths.iter().sum::<usize>() + (num_cols * 3) + 1;
        if total_width > tw {
            let mut excess = total_width as i32 - tw as i32;
            while excess > 0 {
                let mut widest_idx = 0;
                let mut widest_w = col_widths[0];
                for (i, &w) in col_widths.iter().enumerate().skip(1) {
                    if w > widest_w {
                        widest_w = w;
                        widest_idx = i;
                    }
                }
                if widest_w <= 3 {
                    break;
                }
                col_widths[widest_idx] -= 1;
                excess -= 1;
            }
        }
    }

    // Top border
    output.push('┌');
    for (i, &w) in col_widths.iter().enumerate().take(num_cols) {
        output.push_str(&"─".repeat(w + 2));
        if i < num_cols - 1 {
            output.push('┬');
        }
    }
    output.push('┐');
    output.push('\n');

    // Header
    if let Some(ref headers) = table.headers {
        output.push('│');
        for (i, cell) in headers.iter().enumerate().take(num_cols) {
            output.push(' ');
            output.push_str(&fit_header(palette, cell, col_widths[i]));
            output.push(' ');
            output.push('│');
        }
        output.push('\n');

        // Header separator
        output.push('├');
        for (i, &w) in col_widths.iter().enumerate().take(num_cols) {
            output.push_str(&"─".repeat(w + 2));
            if i < num_cols - 1 {
                output.push('┼');
            }
        }
        output.push('┤');
        output.push('\n');
    }

    // Rows; odd data rows get the zebra background when enabled.
    let empty = Cell::text("");
    for (r, row) in table.rows.iter().enumerate() {
        let zebra = r % 2 == 1;
        output.push('│');
        for (i, &width) in col_widths.iter().enumerate().take(num_cols) {
            output.push_str(&palette.filler(" ", zebra));
            let cell = row.get(i).unwrap_or(&empty);
            output.push_str(&fit_cell(palette, cell, width, zebra));
            output.push_str(&palette.filler(" ", zebra));
            output.push('│');
        }
        output.push('\n');
    }

    // Bottom border
    output.push('└');
    for (i, &w) in col_widths.iter().enumerate().take(num_cols) {
        output.push_str(&"─".repeat(w + 2));
        if i < num_cols - 1 {
            output.push('┴');
        }
    }
    output.push('┘');
    output.push('\n');

    output
}
