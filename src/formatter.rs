use unicode_width::UnicodeWidthStr;

pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    if display_width(s) <= max_width {
        return s.to_string();
    }

    if max_width == 0 {
        return String::new();
    }

    if max_width <= 3 {
        return ".".repeat(max_width);
    }

    let mut result = String::new();
    let mut width = 0;
    for ch in s.chars() {
        let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width > max_width - 3 {
            break;
        }
        result.push(ch);
        width += ch_width;
    }
    result.push_str("...");
    result
}
