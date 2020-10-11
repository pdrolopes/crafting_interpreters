use super::ast_printer::ASTPrinter;
use super::interpreter;
use super::parser::Parser;
use super::scanner::Scanner;
use super::token::Token;
use super::token_type::TokenType;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);
static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);

pub fn run_file(path: String) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    run(buffer);

    if HAD_ERROR.load(Ordering::Relaxed) {
        Err("Some error occured".into())
    } else {
        Ok(())
    }
}

pub fn run_prompt() {
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap(); // print! needs to flush so it appears on screen
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.len() <= 1 {
                    // if input has only \n
                    break;
                }
                run(input);
                HAD_ERROR.store(false, Ordering::Relaxed);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

pub fn run(input: String) {
    let mut scanner = Scanner::new(input);
    scanner.scan_tokens();
    // rust vec to ref slice of ref HORRIBLE
    let temp: Vec<_> = scanner.tokens.iter().collect();
    let mut parser = Parser::new(&temp);
    // let expr = parser.parse();

    let expr = match parser.parse() {
        Ok(x) => x,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    match interpreter::interpret(&expr) {
        Ok(x) => println!("{}", x),
        Err(err) => println!("{}", err),
    };

    // println!("{}", ASTPrinter::print(&expr));
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn error_token(token: Token, message: &str) {
    match token.kind {
        TokenType::Eof => report(token.line, "at end", message),
        _ => report(token.line, &format!(" at '{}'", token.lexeme), message),
    }
}

fn report(line: usize, location: &str, message: &str) {
    println!("[line {} ] Error {} : {}", line, location, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}
