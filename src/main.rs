use std::{io::Write, process::ExitCode};

use bf::machine::Machine;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            run_prompt();
            ExitCode::SUCCESS
        }
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: bf [script]");
            ExitCode::from(64)
        }
    }
}

fn run_file(path: &str) -> ExitCode {
    let source = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("could not read {path}: {e}");
        std::process::exit(74); // IO Error
    });

    let source: Vec<char> = source.chars().collect();

    if let Err(e) = run(source) {
        eprintln!("{e}");
        return ExitCode::from(65);
    }

    ExitCode::SUCCESS
}

fn run_prompt() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();

        match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let chars: Vec<char> = line.chars().collect();
                if let Err(e) = run(chars) {
                    eprintln!("{e}");
                }
            }
            Err(e) => {
                eprintln!("IO Error {e}")
            }
        }
    }
}

fn run(source: Vec<char>) -> std::io::Result<()> {
    let mut machine = Machine::new(source);
    machine.run()?;
    Ok(())
}
