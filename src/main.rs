use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::BufReader;

/// Search for a patern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

impl Cli {
    /// Validates the Cli struct, ensuring `pattern` is not empty
    pub fn validate(&self) -> Result<()> {
        if self.pattern.is_empty() {
            Err(anyhow!("pattern must not be empty"))
        } else {
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    // the parse() method is provided by the clap::Parser trait
    // don't use this function outside of main()!
    let args = Cli::parse();
    args.validate()?;

    // let file = File::open(&args.path)?;
    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let reader = BufReader::new(file);

    grrs::find_matches(reader, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn test_find_match() {
        let mut result = Vec::new();
        let input = "lorem ipsum\ndolor sit amet";
        let cursor = Cursor::new(input);
        let _ = grrs::find_matches(cursor, "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}
