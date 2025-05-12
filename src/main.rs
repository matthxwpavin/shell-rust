#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    // Uncomment this block to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("Could not read a command: {:?}", err);
            process::exit(1);
        }

        let inputs: Vec<&str> =
            input.trim_end_matches("\n").split(" ").collect();
        let command = inputs[0];
        if command == "exit" {
            println!("{}", inputs[1]);
            process::exit(inputs[1].parse::<i32>().unwrap());
        }
        println!("{}: command not found", input.trim());
    }
}
