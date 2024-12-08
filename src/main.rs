use std::io::stdout;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;

fn input_to_args(input: &str) -> Vec<&str> {
    input.trim().split_whitespace().collect()
}

fn read_input(input: &str) -> Vec<String> {
    let mut input_vec = Vec::new();
    let mut word = String::new();
    let mut in_quote = false;
    for c in input.trim().chars() {
        match c {
            ' ' if !in_quote => {
                if !word.is_empty() {
                    input_vec.push(word);
                    word = String::new();
                }
            }
            '\'' => {
                in_quote = !in_quote;
            }
            _ => {
                word.push(c);
            }
        }
    }
    if !word.is_empty() {
        input_vec.push(word);
    }
    input_vec
}

fn find_path(command: &str) -> Option<String> {
    let path = env::var("PATH").unwrap_or_else(|_| "".to_string());
    let paths = path.split(":").collect::<Vec<&str>>();
    for path in paths {
        let path = format!("{}/{}", path, command);
        if Path::new(&path).exists() {
            return Some(path);
        }
    }
    None
}

fn normalize_path(path: &str) -> String {
    path.replacen("~", env::var("HOME").unwrap().as_str(), 1)
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
                let args = read_input(&input).get(1..).unwrap().join(" ");
                println!("{}", args);
            }
            "type" => {
                let command = read_input(&input).get(1).unwrap().to_owned();
                if command == "echo" || command == "type" || command == "exit" || command == "pwd" || command == "cd" {
                    println!("{} is a shell builtin", command);
                } else {
                    let path = find_path(command.as_str());
                    match path {
                        Some(path) => println!("{} is {}", command, path),
                        None => println!("{}: not found", command),
                    }
                }
            }
            "exit" => break,
            "pwd" => {
                let path = std::env::current_dir().unwrap();
                println!("{}", path.display());
            }
            "cd" => {
                let current_path = std::env::current_dir().unwrap();
                let new_path = read_input(&input).get(1).unwrap().to_owned();
                let path = current_path.join(normalize_path(&new_path));
                if std::env::set_current_dir(&path).is_err() {
                    println!("cd: {}: No such file or directory", path.display());
                }
            }
            _ => {
                let input_vec = read_input(&input);
                let program = input_vec.get(0).unwrap().to_owned();
                let args = input_vec.into_iter().skip(1);
                let path = find_path(&program);
                match path {
                    Some(path) => {
                        let child_output = std::process::Command::new(path)
                            .args(args)
                            .output()
                            .expect("failed to execute process");
                        print!("{}", String::from_utf8_lossy(&child_output.stdout));
                    }
                    None => println!("{}: command not found", program),
                }
            }
        }
    }
}
