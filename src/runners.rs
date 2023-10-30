use std::{
    fs,
    io::{self, Write},
};

use crate::brainfuck::{Interpreter, Lexer};

pub fn run_repl() {
    let mut input = String::new();
    let mut lexer = Lexer::default();
    let mut interpreter = Interpreter::default();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Invalid input");
        if input.is_empty() || input.trim().ends_with(';') {
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

pub fn run_file(fname: &str) {
    let src_code = fs::read_to_string(fname)
        .unwrap_or_else(|e| panic!("Cannot read file {}. Error: {}", fname, e));
    let mut lexer = Lexer::from(&src_code);
    let mut interpreter = Interpreter::default();
    if let Err(e) = interpreter.interpret(&mut lexer) {
        eprintln!("Program exited with error: {:?}", e);
    }
}
