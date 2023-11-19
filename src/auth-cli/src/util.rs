use std::io::{self, Write};

/// Asks user input and returns it trimmed.
pub fn ask_user_input(prompt: &str) -> io::Result<String> {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush()?;
    if let Err(e) = io::stdin().read_line(&mut input) {
        return Err(e);
    }
    let input = input.trim().to_string();

    Ok(input)
}