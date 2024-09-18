use std::{
    io::{self, BufRead, Write},
    process::exit,
};

use anyhow::{Error, Result};
mod lexer;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: {} [filename]", args[0]);
        exit(1);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("ERROR: {}", e);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(e) => {
                eprintln!("ERROR: {}", e);
                exit(1);
            }
        }
    }

    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    let source = std::fs::read_to_string(path)?;
    run(&source)?;
    Ok(())
}

fn run(buffer: &str) -> Result<()> {
    //let mut lexer = lexer::Lexer::new(source);
    //let tokens = lexer.scan_tokens();
    //for token in tokens {
    //    println!("{:?}", token);
    //}
    Ok(())
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("ERROR: could not flush stdout".to_string()),
        }

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(line) => {
                if line <= 1 {
                    return Ok(());
                }
            }
            Err(_) => return Err("ERROR: could not read line".to_string()),
        }

        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}
