use colored::Colorize;
use std::env;
use std::fs;
use std::process::ExitCode;

mod parser;
mod tokenizer;

use crate::parser::parse;

fn main() -> ExitCode {
    let mut has_encountered_error = false;

    for file_name in env::args().skip(1) {
        let Ok(string) = fs::read_to_string(&file_name) else {
            eprintln!("{} Can not read file {}", "X".red(), file_name);
            has_encountered_error = true;
            continue;
        };

        println!("{} Parsing {}", "i".bright_blue(), file_name);

        match parse(&string) {
            Ok(()) => {
                println!("{} Ok", "✓".green());
            }
            Err(error) => {
                eprintln!("{} {}", "X".red(), error);
                has_encountered_error = true
            }
        }
    }

    if has_encountered_error {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
