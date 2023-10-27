use std::{env, fs};

use brainfuck_rs::{Interpreter, Lexer};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Expected file name as only argument. Got '{:?}' as arguments instead.",
            &args[1..]
        );
        return;
    }
    let fname = args[1].as_str();
    let src_code =
        fs::read_to_string(fname).unwrap_or_else(|_| panic!("Cannot read file {}", fname));
    let lexer = Lexer::new(src_code);
    let mut interpreter = Interpreter::new(lexer);
    if let Err(e) = interpreter.interpret() {
        eprintln!("Program exited with error: {:?}", e);
    }
}
