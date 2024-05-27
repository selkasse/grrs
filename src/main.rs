use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
//use anyhow::{Context, Result};

/// Search for a patern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf, 
}

fn main() -> std::io::Result<()> {

    // the parse() method is provided by the clap::Parser trait
    // don't use this function outside of main()!
    let args = Cli::parse();

    let file = File::open(&args.path)?;
        //.with_context(|| format!("could not read file `{}`", args.path.display()));

    let reader = BufReader::new(file);

    for line in reader.lines(){

        // Handle potential errors from lines iterator
        let line = line?;
        if line.contains(&args.pattern){
            println!("{}", line);
        }
    }

    Ok(())
    
}
