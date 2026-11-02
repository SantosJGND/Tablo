use crate::formatter::{display_width, pad_to_width, truncate_to_width};
use crate::table::Table;
use crate::terminal::get_terminal_width;

pub fn render(table: &Table) -> String {
    let mut output = String::new();
    let term_width = get_terminal_width();

    // Collect all rows for width calculation
    let mut all_rows: Vec<Vec<String>> = Vec::new();
    if let Some(ref headers) = table.headers {
        all_rows.push(headers.clone());
    }
    all_rows.extend(table.rows.iter().cloned());

    if all_rows.is_empty() {
        return output;
    }

    let num_cols = all_rows[0].len();
    let mut col_widths = vec![0usize; num_cols];

    for row in &all_rows {
        for (i, cell) in row.iter().enumerate().take(num_cols) {
            let w = display_width(cell);
            if w > col_widths[i] {
                col_widths[i] = w;
            }
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
            .map(|(i, cell)| {
                let truncated = truncate_to_width(cell, col_widths[i]);
                pad_to_width(&truncated, col_widths[i])
            })
            .collect();
        output.push_str(&parts.join("  "));
        output.push('\n');
    }

    // Render rows
    for row in &table.rows {
        let parts: Vec<String> = (0..num_cols)
            .map(|i| {
                let cell = row.get(i).map(|s| s.as_str()).unwrap_or("");
                let truncated = truncate_to_width(cell, col_widths[i]);
                pad_to_width(&truncated, col_widths[i])
            })
            .collect();
        output.push_str(&parts.join("  "));
        output.push('\n');
    }

    output
}
