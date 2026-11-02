use terminal_size::{terminal_size, Width};

pub fn get_terminal_width() -> Option<usize> {
    if let Some((Width(w), _)) = terminal_size() {
        Some(w as usize)
    } else {
        None
    }
}
