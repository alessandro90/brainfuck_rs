use std::{
    fs,
    io::{self, Write},
};

use crate::brainfuck::{Interpreter, Lexer};

pub fn run_repl() {
    let mut input = String::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Invalid input");
        if input.is_empty() || input.ends_with(';') {
            continue;
        }
        let lexer = Lexer::new(input.clone());
        input.clear();
        let mut interpreter = Interpreter::new(lexer);
        if let Err(e) = interpreter.interpret() {
            eprintln!("Invalid input code: {:?}", e);
        }
    }
}

pub fn run_file(fname: &str) {
    let src_code =
        fs::read_to_string(fname).unwrap_or_else(|_| panic!("Cannot read file {}", fname));
    let lexer = Lexer::new(src_code);
    let mut interpreter = Interpreter::new(lexer);
    if let Err(e) = interpreter.interpret() {
        eprintln!("Program exited with error: {:?}", e);
    }
}
