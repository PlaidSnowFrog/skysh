use std::env;
use std::io::{self, Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub enum Input {
    Line(Vec<String>), // user pressed Enter; line already split into tokens
    Interrupt,         // Ctrl+C
    Eof,               // Ctrl+D on empty line
}

pub fn prompt() -> io::Result<Input> {
    print!("{:?} > ", env::current_dir().unwrap().to_str().unwrap());
    io::stdout().flush()?;

    enable_raw_mode()?;
    let result = read_keys();
    disable_raw_mode()?; // always restore the terminal, even if reading failed
    print!("\r\n");
    result
}

fn read_keys() -> io::Result<Input> {
    let mut buf = String::new();
    let mut out = io::stdout();

    loop {
        let Event::Key(key) = event::read()? else { continue };
        if key.kind != KeyEventKind::Press { continue; }
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        match key.code {
            KeyCode::Char('c') if ctrl => return Ok(Input::Interrupt),
            KeyCode::Char('d') if ctrl && buf.is_empty() => return Ok(Input::Eof),
            KeyCode::Enter => {
                let tokens = buf.split_whitespace().map(String::from).collect();
                return Ok(Input::Line(tokens));
            }
            KeyCode::Backspace => {
                if buf.pop().is_some() {
                    write!(out, "\x08 \x08")?; // move back, erase char, move back
                }
            }
            KeyCode::Char(c) if !ctrl => {
                buf.push(c);
                write!(out, "{c}")?; // raw mode doesn't echo, so we do it
            }
            _ => {}
        }
        out.flush()?;
    }
}