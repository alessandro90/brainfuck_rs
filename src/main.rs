use std::env;

use brainfuck_rs::runners::{run_file, run_repl};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        run_repl();
    } else {
        let fname = args[1].as_str();
        run_file(fname);
    }
}
