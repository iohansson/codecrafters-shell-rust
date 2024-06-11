use std::io::stdout;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // read PATH environment variable
    let path = std::env::var("PATH").unwrap_or_else(|_| "".to_string());
    let paths = path.split(":").collect::<Vec<&str>>();
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
            "type" => {
                let command = input.trim().split_whitespace().skip(1).next().unwrap();
                if command == "echo" || command == "type" || command == "exit" {
                    println!("{} is a shell builtin", command);
                } else {
                    let mut found = false;
                    for path in &paths {
                        let path = format!("{}/{}", path, command);
                        if std::path::Path::new(&path).exists() {
                            println!("{} is {}", command, path);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println!("{}: not found", command);
                    }
                }
            }
            "exit" => break,
            _ => println!("{}: command not found", command),
        }
    }
}
