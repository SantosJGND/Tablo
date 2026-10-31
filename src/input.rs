use anyhow::{Context, Result};
use std::io::{self, Read};

pub fn read_input(file: Option<&str>) -> Result<String> {
    match file {
        Some(path) => {
            std::fs::read_to_string(path)
                .with_context(|| format!("Failed to read file: {}", path))
        }
        None => {
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .context("Failed to read from stdin")?;
            Ok(input)
        }
    }
}
