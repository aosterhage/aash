use std::io::{Result, Write};
use std::write;

/// The default prompt to output when one hasn't been configured.
const DEFAULT: &str = "$ ";

/// Writes the prompt to `writer` using the `std::io::Write` trait.
///
/// `write` will internally buffer writes to `writer` and make only
/// a single call to `flush` just before returning.
pub fn write<T: Write>(writer: &mut T) -> Result<()> {
    let mut writer = std::io::BufWriter::new(writer);

    write!(writer, "{}", DEFAULT)?;

    // Since we have no guarantees on when `writer` will flush
    // its contents, manually flush now.
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod write {
    use super::write;

    #[test]
    fn default() {
        let mut buffer = vec![];
        write(&mut buffer).unwrap();
        assert_eq!(str::from_utf8(&buffer).unwrap(), super::DEFAULT);
    }
}
