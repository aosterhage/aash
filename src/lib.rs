use std::io::{BufRead, BufReader};
use std::io::{stderr, stdin, stdout};

mod evaluator;
mod parser;
mod prompt;

pub fn main() {
    // Write the prompt out to `stdout`.
    prompt::write(&mut stdout()).unwrap();

    // Read an input line from `stdin`.
    let mut line = String::new();
    BufReader::new(&mut stdin()).read_line(&mut line).unwrap();

    // Parse the input line.
    let parsed_input = &parser::parse(&line)[..];

    // The `command` is the first word parsed from the input.
    let command = match parsed_input.len() {
        0 => "",
        _ => &parsed_input[0],
    };
    // `argv` is the remaining words in the parsed input.
    let argv = match parsed_input.len() {
        0..=1 => &[],
        2.. => &parsed_input[1..],
    };

    // Evaluate the input line and output to `stdout` and `stderr`.
    evaluator::eval(command, argv, &mut stdout(), &mut stderr()).unwrap();
}
