pub mod ast_printer;
pub mod error;
mod expr;
mod lox;
pub mod parser;
mod scanner;
pub mod token;
pub mod token_type;

use std::env;
fn main() {
    let args = env::args();

    // First argument is binary name
    match args.len() {
        1 => {
            lox::run_prompt();
        }
        2 => {
            lox::run_file(args.last().unwrap());
        }
        _ => {
            println!("Usage: jlox [script]");
            // EX_USAGE (64)	   The command was used incorrectly, e.g., with the
            // wrong number of arguments, a bad flag, a bad syntax
            // in a parameter, or whatever.
            std::process::exit(64);
        }
    };
}
