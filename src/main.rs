#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    // Wait for user input
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        let input_parts: Vec<&str> = input.split(" ").collect();

        let command = input_parts[0];
        let params = &input_parts[1..];

        if command == "exit" {
            break;
        } else if command == "echo" {
            let output = params.join(" ");
            println!("{output}");
        } else {
            println!("{}: command not found", input);
        }
    }
}
