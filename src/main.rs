use std::io::Write as _;
use std::io::{stderr, stdin, stdout};
use std::{write, writeln};

use aash::input::parser;
use aash::output::prompt;

fn main() {
    // Create two buffers, one for input, one for output.
    let mut input_buffer = String::new();
    let mut output_buffer = String::new();

    // Write the prompt out to stdout.
    prompt::write(&mut output_buffer).unwrap();
    write!(&mut stdout(), "{}", output_buffer).unwrap();
    // As the documentation for `Stdout` says:
    // > By default, the handle is line-buffered when connected to a terminal [...]
    // Manually call flush since the prompt never contains a newline character.
    stdout().flush().unwrap();

    // Read and parse an input line.
    stdin().read_line(&mut input_buffer).unwrap();
    let parsed_input = &parser::parse(input_buffer.as_str())[..];

    // The `command` is the first word parsed from the input.
    // `argv` is the remaining words in the parsed input.
    let command = &parsed_input[0];
    let _argv = &parsed_input[1..];

    // If we don't know what the command is, print an error to stderr.
    writeln!(&mut stderr(), "{}: command not found", command).unwrap();
}
