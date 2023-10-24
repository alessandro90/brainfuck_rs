use std::{env, fs};

use brainfuck_rs::interpret;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Expected file name as only argument. Got {} arguments instead.",
            args.len() - 1
        );
        return;
    }
    let fname = &args[1];
    let src_code =
        fs::read_to_string(fname).unwrap_or_else(|_| panic!("Cannot read file {}", fname));
    match interpret(src_code) {
        Ok(_) => println!("Program completed without errors."),
        Err(e) => eprintln!("Program exited with error: {:?}", e),
    };
}
