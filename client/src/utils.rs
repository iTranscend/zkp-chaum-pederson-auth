use std::io::{self, Write};

use zkp_utils::style;

pub fn maybe_input(value: Option<String>, prompt: &str) -> anyhow::Result<String> {
    match value {
        Some(val) => Ok(val),
        None => {
            print!(
                "{}{}{} {} {}",
                style::fg::YELLOW,
                "[?]",
                style::fg::RESET,
                prompt,
                style::fg::CYAN
            );
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            print!("{}", style::fg::RESET);
            Ok(input.trim().to_string())
        }
    }
}

pub fn maybe_password(value: Option<String>, prompt: &str) -> anyhow::Result<String> {
    match value {
        Some(val) => Ok(val),
        None => {
            print!(
                "{}{}{} {} ",
                style::fg::YELLOW,
                "[?]",
                style::fg::RESET,
                prompt
            );
            io::stdout().flush()?;
            Ok(rpassword::read_password()?)
        }
    }
}
