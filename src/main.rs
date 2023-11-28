use std::{env, process::ExitCode};

use brainfuck_rs::runners::{run_file, run_repl};

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        run_repl();
        return ExitCode::SUCCESS;
    }
    if args.len() > 2 {
        eprintln!("Unused command arguments: {:?}", &args[2..]);
        return ExitCode::FAILURE;
    }
    let fname = args[1].as_str();
    run_file(fname)
}
