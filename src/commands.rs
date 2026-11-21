use std::env;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::{exit, Command, Stdio};

pub fn check_command(input: Vec<String>) {
    match input[0].as_str() {
        "cd" => {
            // Check if a directory was provided
            if input.len() >= 2 {
                if let Err(e) = env::set_current_dir(&input[1]) {
                    eprintln!("Could not change working directory: {}", e);
                }
            } else {
                match env::home_dir() {
                    Some(home) => if let Err(e) = env::set_current_dir(home) {
                        eprintln!("{}", e);
                    },
                    None => eprintln!("HOME not set"),
                }
            }
        }
        "exit" => {
            // Check if an exit code was provided
            if input.len() >= 2 {
                exit(input[1].parse::<i32>().unwrap_or_else(|_| {
                    eprintln!("skysh: exit: {}: numeric argument required", input[1]);
                    2
                }));
            } else {
                // exit with code 0
                exit(0);
            }
        }
        "help" => {
            println!("Commands:");
            println!("  cd <directory> | change working directory");
            println!("  exit <code>    | exit the program with exit code <code> or 0 \
            if not provided");
        }
       _ => {
           match Command::new(&input[0])
               .args(&input[1..])
               .env("SHELL", env::current_exe().expect("Could not get shell executable path"))
               .status()
           {
               Ok(status) => { /* keep status.code() for $? later */ }
               Err(e) => eprintln!("skysh: {}: {}", input[0], e),
           }
       },
    }
}