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

fn find_matches<R: BufRead>(reader: R, pattern: &str) -> Result<()> {
    for line in reader.lines() {
        // Handle potential errors from lines iterator
        let line = line?;
        if line.contains(pattern) {
            println!("{}", line);
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

    find_matches(reader, &args.pattern)?;

    Ok(())
}
