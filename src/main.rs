use crate::commands::check_command;
use crate::input::{prompt, Input};

pub mod input;
pub mod commands;
pub mod signal_handling;

fn main() -> std::io::Result<()> {
    signal_handling::install()?; // so SIGINT doesn't kill the shell while a child runs

    loop {
        match prompt()? {
            Input::Line(tokens) => check_command(tokens),
            Input::Interrupt => println!("^C"), // drop the line, loop shows a new prompt
            Input::Eof => break,                // Ctrl+D on empty line
            _ => {panic!("Unexpected input")}
        }
    }
    Ok(())
}
