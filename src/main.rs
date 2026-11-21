use crate::input::prompt;

pub mod input;
pub mod commands;

fn main() {
    let tokens = prompt();

    for token in tokens {
        println!("{}", token);
    }
}
