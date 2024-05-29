use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Search for a patern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

// Generic Type `R` allows the function to accept any reader that implements the `BufRead` trait
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

fn main() -> Result<()> {
    // the parse() method is provided by the clap::Parser trait
    // don't use this function outside of main()!
    let args = Cli::parse();

    // let file = File::open(&args.path)?;
    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let reader = BufReader::new(file);

    find_matches(reader, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_find_match() {
        let mut result = Vec::new();
        let input = "lorem ipsum\ndolor sit amet";
        let cursor = Cursor::new(input);
        let _ = find_matches(cursor, "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}
