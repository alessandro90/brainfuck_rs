use std::env;

use brainfuck_rs::runners::{run_file, run_repl};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        run_repl();
    } else {
        if args.len() > 2 {
            eprintln!("Unused command arguments: {:?}", &args[2..]);
        }
        let fname = args[1].as_str();
        run_file(fname);
    }
}
