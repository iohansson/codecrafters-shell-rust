use std::io::stdout;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
}
