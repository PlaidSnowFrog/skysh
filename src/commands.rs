use std::env;
use std::process::exit;

pub fn check_command(input: Vec<String>) {
    match input[0].as_str() {
        "cd" => {
            // Check if a directory was provided
            if input.len() >= 2 {
                env::set_current_dir(&input[1])
                    .expect("Could not change working directory");
            } else {
                panic!("Provide a directory");
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
       _ => panic!("Unknown command!"),
    }
}