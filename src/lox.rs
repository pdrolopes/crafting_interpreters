use super::interpreter::Interpreter;
use super::parser::ParseResult;
use super::parser::Parser;
use super::scanner::Scanner;
use super::token::Token;
use super::token_type::TokenType;
use crate::error::LoxError;
use crate::resolver::Resolver;
use crate::stmt::Stmt;
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
    let stmts = run(buffer);
    let depth_map = Resolver::new().run(&stmts).map_err(|err| {
        println!("{}", err);
        err
    })?;
    let mut interpreter = Interpreter::new();
    interpreter.add_expr_ids_depth(depth_map);
    interpreter.interpret(&stmts);

    if HAD_ERROR.load(Ordering::Relaxed) {
        Err("Some error occured".into())
    } else {
        Ok(())
    }
}

pub fn run_prompt() {
    let mut interpreter = Interpreter::new();
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
                let stmts = repl_interpret(input);
                match stmts {
                    ReplStatements::List(x) => {
                        Resolver::new()
                            .run(&x)
                            .map(|map| interpreter.add_expr_ids_depth(map))
                            .unwrap(); // TODO Add error treatment to prompt function
                        interpreter.interpret(&x);
                    }
                    ReplStatements::SingleExpr(x) => interpreter.print(&x),
                };
                HAD_ERROR.store(false, Ordering::Relaxed);
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

pub enum ReplStatements {
    SingleExpr(Stmt),
    List(Vec<Stmt>),
}

pub fn repl_interpret(input: String) -> ReplStatements {
    let mut scanner = Scanner::new(input);
    scanner.scan_tokens();
    let mut parser = Parser::new(&scanner.tokens, true);
    let parsed_result = parser.parse();

    let errs: Vec<_> = match &parsed_result {
        ParseResult::SingleExpr(Err(x)) => vec![x.clone()],
        ParseResult::SingleExpr(_) => vec![],
        ParseResult::List(x) => x
            .into_iter()
            .filter_map(|x| x.as_ref().err())
            .cloned()
            .collect::<Vec<LoxError>>(),
    };

    if !errs.is_empty() {
        errs.iter().for_each(|err| println!("{}", err));
        return ReplStatements::List(vec![]);
    }

    match parsed_result {
        ParseResult::List(x) => {
            ReplStatements::List(x.into_iter().filter_map(|x| x.ok()).collect())
        }
        ParseResult::SingleExpr(stmt) => {
            if let Ok(stmt) = stmt {
                ReplStatements::SingleExpr(stmt)
            } else {
                ReplStatements::List(vec![])
            }
        }
    }
}

// TODO figureout duplicated code
pub fn run(input: String) -> Vec<Stmt> {
    let mut scanner = Scanner::new(input);
    scanner.scan_tokens();
    let mut parser = Parser::new(&scanner.tokens, false);
    let parsed_result = parser.parse();

    let list_result = match parsed_result {
        ParseResult::List(x) => x,
        ParseResult::SingleExpr(_) => unreachable!(), // Interpreting a file doesnt allow expr only without ;,
    };

    let errs: Vec<_> = list_result
        .iter()
        .filter_map(|x| x.as_ref().err())
        .collect();

    if !errs.is_empty() {
        errs.iter().for_each(|err| println!("{}", err));
        return vec![];
    }

    list_result.into_iter().filter_map(|x| x.ok()).collect()
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

pub fn report_runtime(err: LoxError) {
    println!("{}", err);
    HAD_RUNTIME_ERROR.store(true, Ordering::Relaxed);
}
