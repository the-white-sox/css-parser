use std::env;
use std::fs;
use std::process::ExitCode;

mod parser;
mod tokenizer;

use crate::parser::parse;

const GREEN_CHECK: &str = "\u{001b}[92m✓\u{001b}[0m";
const RED_X: &str = "\u{001b}[31mX\u{001b}[0m";
const BLUE_I: &str = "\u{001b}[96mi\u{001b}[0m";

fn main() -> ExitCode {
    let mut has_encountered_error = false;

    for file_name in env::args().skip(1) {
        let Ok(string) = fs::read_to_string(&file_name) else {
            eprintln!("{} Can not read file {}", RED_X, file_name);
            has_encountered_error = true;
            continue;
        };

        println!("{} Parsing {}", BLUE_I, file_name);

        match parse(&string) {
            Ok(()) => {
                println!("{} Ok", GREEN_CHECK);
            }
            Err(error) => {
                eprintln!("{} {}", RED_X, error);
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
