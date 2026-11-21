use crate::commands::check_command;
use crate::input::prompt;

pub mod input;
pub mod commands;

fn main() {
    loop {
        let tokens = prompt();

        check_command(tokens);
    }
}
