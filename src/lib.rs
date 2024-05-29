// Generic Type `R` allows the function to accept any reader that implements the `BufRead` trait

use anyhow::Result;
use std::io::BufRead;

pub fn find_matches<R: BufRead>(
    reader: R,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<()> {
    for line in reader.lines() {
        // Handle potential errors from lines iterator
        let line = line?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}
