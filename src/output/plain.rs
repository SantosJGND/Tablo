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
        let total_width: usize =
            col_widths.iter().sum::<usize>() + (num_cols.saturating_sub(1)) * 2;
        if total_width > tw {
            // Truncate widest columns
            let mut excess = total_width as i32 - tw as i32;
            while excess > 0 {
                // Find widest column
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

    // Render header
    if let Some(ref headers) = table.headers {
        let parts: Vec<String> = headers
            .iter()
            .enumerate()
            .take(num_cols)
            .map(|(i, cell)| fit_header(palette, cell, col_widths[i]))
            .collect();
        output.push_str(&parts.join("  "));
        output.push('\n');
    }

    // Render rows; odd data rows get the zebra background when enabled.
    let empty = Cell::text("");
    for (r, row) in table.rows.iter().enumerate() {
        let zebra = r % 2 == 1;
        let parts: Vec<String> = (0..num_cols)
            .map(|i| {
                let cell = row.get(i).unwrap_or(&empty);
                fit_cell(palette, cell, col_widths[i], zebra)
            })
            .collect();
        output.push_str(&parts.join(&palette.filler("  ", zebra)));
        output.push('\n');
    }

    output
}
