use std::{
    fs,
    io::{self, Write},
    process::ExitCode,
};

use crate::brainfuck::{Interpreter, Lexer};

pub fn run_repl() {
    let mut input = String::new();
    let mut lexer = Lexer::default();
    let mut interpreter = Interpreter::default();
    let mut line_nr = 0;
    loop {
        print!("[{}] ", line_nr);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Invalid input");
        if input.is_empty() || input.trim().ends_with(';') {
            line_nr += 1;
            continue;
        }
        lexer.append(&input);
        input.clear();
        if let Err(e) = interpreter.interpret(&mut lexer) {
            eprintln!("Invalid input code: {:?}. Resetting interpreter", e);
            // TODO: we lost everything here. It would be better to just
            // go back to the previous valid state
            lexer = Lexer::default();
            interpreter = Interpreter::default();
        }
    }
}

pub fn run_file(fname: &str) -> ExitCode {
    match fs::read_to_string(fname) {
        Err(e) => {
            eprintln!("Cannot read file {}. Error: {}", fname, e);
            ExitCode::FAILURE
        }
        Ok(src_code) => {
            let mut lexer = Lexer::from(&src_code);
            let mut interpreter = Interpreter::default();
            if let Err(e) = interpreter.interpret(&mut lexer) {
                eprintln!("Program exited with error: {:?}", e);
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
    }
}
