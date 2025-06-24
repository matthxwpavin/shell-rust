#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::split_paths,
    process::{self, Command},
};

fn main() {
    // Uncomment this block to pass the first stage

    let built_ins = ["exit", "echo", "type"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("Could not read a command: {:?}", err);
            process::exit(1);
        }

        let input = input.trim();
        let inputs: Vec<&str> = input.split(" ").collect();
        let command = inputs[0];

        let find_command_path = |command| {
            if let Some(paths) = std::env::var_os("PATH") {
                split_paths(&paths)
                    .map(|path| path.join(command))
                    .find(|path| path.is_file())
            } else {
                None
            }
        };

        match command {
            "exit" => process::exit(inputs[1].parse::<i32>().unwrap()),
            "echo" => {
                let mut s = String::new();
                for arg in &inputs[1..] {
                    s.push_str(&format!("{} ", arg));
                }
                println!("{}", s.trim());
            }
            "type" => {
                let bin = if inputs.len() > 1 { inputs[1] } else { "" };
                if built_ins.contains(&bin) {
                    println!("{} is a shell builtin", bin)
                } else if let Some(path) = find_command_path(bin) {
                    println!("{} is {}", bin, path.to_str().unwrap())
                } else {
                    println!("{}: not found", bin)
                }
            }
            _ => {
                if let Some(path) = find_command_path(command) {
                    let cmd = path.to_str().unwrap();
                    let out = Command::new(cmd)
                        .args(vec![&input[1..]])
                        .output()
                        .expect("could not execute the command");
                    println!(
                        "{}",
                        String::from_utf8(out.stdout)
                            .expect("could not get output")
                    )
                } else {
                    println!("{}: command not found", input.trim())
                }
            }
        }
    }
}
