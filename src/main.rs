#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    // Wait for user input
    let stdin = io::stdin();

    let commands = &["exit", "echo", "type"];

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
        } else if command == "type" {
            if params.len() > 0 {
                let is_builtin = if commands.contains(&params[0]) {
                    "is a shell builtin"
                } else {
                    "not found"
                };
                println!("{} {}", &params[0], is_builtin);
            }
        } else {
            println!("{}: command not found", input);
        }
    }
}
