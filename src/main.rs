use otp::Program;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = Program::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = otp::run(program) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
