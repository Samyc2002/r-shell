use std::{
    env,
    io::{self, Write},
};
use std::{path::Path, process::Command};

fn get_full_path(program_name: &str) -> (bool, String) {
    let path = env::var("PATH").unwrap_or_default();
    let paths: Vec<&str> = path.split(":").collect();

    let mut is_path_var = false;
    let mut exe_path = String::new();
    for path in paths.iter() {
        let mut full_path = Path::new(path).join(program_name);
        full_path.set_extension("");
        if full_path.exists() {
            is_path_var = true;
            exe_path = format!("{}", full_path.display());
            break;
        }
    }

    (is_path_var, exe_path)
}

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
                let is_builtin = commands.contains(&params[0]);
                if is_builtin {
                    println!("{} is a shell builtin", &params[0]);
                    continue;
                }

                let (is_path_var, exe_path) = get_full_path(params[0]);

                if is_path_var {
                    println!("{}", exe_path);
                    continue;
                }

                println!("{} not found", &params[0]);
            }
        } else {
            let (is_path_var, exe_path) = get_full_path(command);
            if is_path_var {
                let mut child = Command::new(exe_path)
                    .args(params)
                    .stdout(io::stdout())
                    .spawn()
                    .unwrap();
                child.wait().unwrap();
            } else {
                println!("{}: command not found", input);
            }
        }
    }
}
