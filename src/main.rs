use std::io::stdout;
#[allow(unused_imports)]
use std::io::{self, Write};

fn input_to_args(input: &str) -> Vec<&str> {
    input.trim().split_whitespace().collect()
}

fn find_path(command: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap_or_else(|_| "".to_string());
    let paths = path.split(":").collect::<Vec<&str>>();
    for path in paths {
        let path = format!("{}/{}", path, command);
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }
    None
}

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();

        stdin.read_line(&mut input).unwrap();

        let command = *input_to_args(&input).get(0).unwrap();

        match command {
            "echo" => {
                // join all the arguments with a space
                let args = input_to_args(&input).get(1..).unwrap().join(" ");
                println!("{}", args);
            }
            "type" => {
                let command = *input_to_args(&input).get(1).unwrap();
                if command == "echo" || command == "type" || command == "exit" {
                    println!("{} is a shell builtin", command);
                } else {
                    let path = find_path(command);
                    match path {
                        Some(path) => println!("{} is {}", command, path),
                        None => println!("{}: not found", command),
                    }
                }
            }
            "exit" => break,
            _ => {
                let program = *input_to_args(&input).get(0).unwrap();
                let args = input_to_args(&input).into_iter().skip(1);
                let path = find_path(program);
                match path {
                    Some(path) => {
                        let child_output = std::process::Command::new(path)
                            .args(args)
                            .output()
                            .expect("failed to execute process");
                        println!("{}", String::from_utf8_lossy(&child_output.stdout));
                    }
                    None => println!("{}: command not found", program),
                }
            }
        }
    }
}
