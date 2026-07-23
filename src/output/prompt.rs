use std::fmt::{Result, Write};
use std::write;

/// The default prompt to output when one hasn't been configured.
const DEFAULT: &str = "$ ";

/// Writes the prompt to `writer` using the `std::fmt::Write` trait.
///
/// Enforce the use of `std::fmt::Write` trait to guarantee that the prompt will only write UTF-8.
/// Callers that wish to forward the written data to a `std::io::Write` writer (such as `std::io::stdout()`)
/// must use an intermediary buffer that supports the `std::fmt::Write` trait such as String.
///
/// # Examples
///
/// ```
/// use std::io::Write as _;
/// use aash::output::prompt;
///
/// let mut buffer = String::new();
/// prompt::write(&mut buffer).unwrap();
/// std::write!(&mut std::io::stdout(), "{}", buffer).unwrap();
/// std::io::stdout().flush().unwrap();
/// ```
pub fn write<T: Write>(writer: &mut T) -> Result {
    write!(writer, "{}", DEFAULT)?;
    Ok(())
}

#[cfg(test)]
mod write {
    #[test]
    /// Ensure the default prompt is output when one hasn't been configured.
    fn default() {
        let mut buffer = String::new();
        super::write(&mut buffer).unwrap();
        assert_eq!(buffer, super::DEFAULT);
    }
}
