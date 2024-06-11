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

        let command = input.trim().split_whitespace().next().unwrap();

        match command {
            "echo" => {
                // join all the arguments with a space
                let args = input
                    .trim()
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join(" ");
                println!("{}", args);
            }
            "exit" => break,
            _ => println!("{}: command not found", command),
        }
    }
}
