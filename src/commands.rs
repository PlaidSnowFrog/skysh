use std::env;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::{exit, Command, Stdio};

pub fn check_command(input: Vec<String>) {
    match input[0].as_str() {
        "cd" => {
            // Check if a directory was provided
            if input.len() >= 2 {
                env::set_current_dir(&input[1])
                    .expect("Could not change working directory");
            } else {
                env::set_current_dir(env::home_dir()
                    .expect("Could not change to home directory"))
                    .expect("Could not change working directory");
            }
        }
        "exit" => {
            // Check if an exit code was provided
            if input.len() >= 2 {
                exit(input[1].parse::<i32>().unwrap());
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