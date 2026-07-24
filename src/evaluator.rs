use std::io::{BufWriter, Write};

/// Evaluates `command` (with `argv`) and returns the corresponding `output` and `error` streams.
///
/// `eval` will internally buffer writes to `output` and `error` and only flush when necessary.
pub fn eval<T: Write, U: Write>(
    command: &str,
    _argv: &[&str],
    output: &mut T,
    error: &mut U,
) -> Result<(), Box<dyn std::error::Error>> {
    // If there is no `command`, don't do anything.
    if command.is_empty() {
        return Ok(());
    }

    let mut _output = BufWriter::new(output);
    let mut error = BufWriter::new(error);

    // If we don't know what the command is, write this to `error`.
    writeln!(error, "{}: command not found", command)?;
    error.flush()?;

    Ok(())
}

#[cfg(test)]
mod eval {
    use super::eval;

    fn test_eval(command: &str, argv: &[&str], expect_output: &str, expect_error: &str) {
        let (mut output, mut error) = (vec![], vec![]);
        eval(command, argv, &mut output, &mut error).unwrap();
        assert_eq!(str::from_utf8(&output).unwrap(), expect_output);
        assert_eq!(str::from_utf8(&error).unwrap(), expect_error);
    }

    #[test]
    fn no_command_without_argv() {
        test_eval("", &[], "", "");
    }

    #[test]
    fn no_command_with_argv() {
        test_eval("", &["some", "args"], "", "");
    }

    #[test]
    fn invalid_command() {
        test_eval("notacommand", &[], "", "notacommand: command not found\n");
    }
}
