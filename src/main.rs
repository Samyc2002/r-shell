#[allow(unused_imports)]
use std::{
    env,
    io::{self, Write},
};

fn main() {
    // Uncomment this block to pass the first stage
    // Wait for user input
    let stdin = io::stdin();

    let commands = &["exit", "echo", "type"];
    let path = env::var("PATH").unwrap_or_default();
    let paths: Vec<&str> = path.split(":").collect();

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
                let is_builtin = commands.contains(&params[0]);
                if is_builtin {
                    println!("{} is a shell builtin", &params[0]);
                    continue;
                }

                let mut is_path_var = false;
                let mut exe_path = String::new();
                for path in paths.iter() {
                    is_path_var = path.ends_with(params[0]);

                    if is_path_var {
                        exe_path = format!("{path}");
                        break;
                    }
                }

                if is_path_var {
                    println!("{} is {}", &params[0], exe_path);
                    continue;
                }

                println!("{} not found", &params[0]);
            }
        } else {
            println!("{}: command not found", input);
        }
    }
}
