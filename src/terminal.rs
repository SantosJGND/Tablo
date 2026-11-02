use std::io::{self, IsTerminal};
use terminal_size::{terminal_size, Width};

pub fn get_terminal_width() -> Option<usize> {
    if let Some((Width(w), _)) = terminal_size() {
        Some(w as usize)
    } else {
        None
    }
}

/// Colour is used only when stdout is a terminal, `--no-color` is not given
/// and `NO_COLOR` is not set to a non-empty value.
pub fn color_enabled(no_color_flag: bool) -> bool {
    let env_no_color = std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty());
    decide_color(no_color_flag, env_no_color, io::stdout().is_terminal())
}

fn decide_color(no_color_flag: bool, env_no_color: bool, stdout_is_terminal: bool) -> bool {
    !no_color_flag && !env_no_color && stdout_is_terminal
}

#[cfg(test)]
mod tests {
    use super::decide_color;

    #[test]
    fn colour_requires_terminal_and_no_opt_out() {
        assert!(decide_color(false, false, true));
        assert!(!decide_color(false, false, false));
        assert!(!decide_color(true, false, true));
        assert!(!decide_color(false, true, true));
    }
}
