use std::io::stdout;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();

        stdin.read_line(&mut input).unwrap();

        // Check if the user wants to exit
        if input.trim() == "exit 0" {
            break;
        }

        println!("{}: command not found", input.trim());
    }
}
