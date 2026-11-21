use std::io;
use std::io::Write;

fn get_user_input() -> String {
    let mut input = String::new();

    let _ = std::io::stdin().read_line(&mut input);
    input
}

fn tokenize(input: String) -> Vec<String> {
    input.trim().split_whitespace().map(|s| s.to_string()).collect()
}

pub fn prompt() -> Vec<String> {
    print!("> ");
    io::stdout().flush().unwrap();

    let input = get_user_input();
    tokenize(input)
}

