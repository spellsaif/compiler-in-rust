mod scanner;

use crate::scanner::*;

use std::{env, fs, io::{self, Write}, process::exit};

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents)
    }
}

fn run(contents: &str) -> Result<(), String>{
    let scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token)
    }
    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    
    loop {

        print!("> ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("could not flush stdout".to_string()),
        }
        match stdin.read_line(&mut buffer) {
            Ok(n) => {
                if n <= 2 {
                    return Ok(());
                }
            },
            Err(_) => return Err("Couln't read line".to_string())
        };

        println!("You wrote {}", buffer);
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        println!("Usage: Yuna [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("error occured: {} ", msg);
                exit(1);
            }
        }
    } else {
       match run_prompt() {
        Ok(_) => exit(0),
        Err(msg) => {
            println!("Error: {}", msg);
            exit(1);
        } 
       }
    }


}
