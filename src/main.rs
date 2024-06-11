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
                let args = input.trim().split_whitespace().skip(1);
                for arg in args {
                    print!("{} ", arg);
                }
                println!();
            }
            "exit" => break,
            _ => println!("{}: command not found", command),
        }
    }
}
