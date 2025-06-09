#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env::split_paths, process};

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
                    println!("{} is a shell builtin", bin);
                    continue;
                }

                let mut found = false;
                if let Some(paths) = std::env::var_os("PATH") {
                    for path in split_paths(&paths) {
                        let path = path.join(bin);
                        if path.is_file() {
                            println!("{} is {}", bin, path.to_str().unwrap());
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    println!("{}: not found", bin);
                }
            }
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
