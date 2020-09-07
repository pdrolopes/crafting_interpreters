use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

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
    // Scanner scanner = new Scanner(source);
    // List<Token> tokens = scanner.scanTokens();

    // // For now, just print the tokens.
    // for (Token token : tokens) {
    //   System.out.println(token);
    // }
    todo!();
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}
fn report(line: usize, location: &str, message: &str) {
    println!("[line {} ] Error {} : {}", line, location, message);
    HAD_ERROR.store(true, Ordering::Relaxed);
}
