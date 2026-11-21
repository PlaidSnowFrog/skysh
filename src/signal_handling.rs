use std::sync::{Arc, atomic::AtomicBool};
use signal_hook::{consts::SIGINT, flag};

pub fn install() -> std::io::Result<()> {
    // Registering any handler is enough: the shell survives SIGINT,
    // and child processes still get the default (die on Ctrl+C) after exec.
    flag::register(SIGINT, Arc::new(AtomicBool::new(false)))?;
    Ok(())
}