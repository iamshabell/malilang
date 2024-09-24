use std::{
    io::{self, BufRead, Write},
    process::exit,
};

use anyhow::{Error, Result};
use environment::Environment;
use interpreter::Interpreter;
use parser::Parser;
mod environment;
mod expr;
mod interpreter;
mod lexer;
mod parser;
mod statement;

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
    let mut interpreter = Interpreter::new();
    let source = std::fs::read_to_string(path)?;
    run(&mut interpreter, &source)?;
    Ok(())
}

fn run(interpreter: &mut Interpreter, buffer: &str) -> Result<()> {
    let mut lexer = lexer::Lexer::new(buffer);
    let tokens = lexer.lex()?;

    let mut ast = Parser::new(tokens);
    let statements = ast.parse()?;

    interpreter.interpret(statements)?;

    Ok(())
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter = Interpreter::new();
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

        match run(&mut interpreter, &buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}
