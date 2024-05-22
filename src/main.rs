use clap::Parser;

/// Search for a patern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    // PathBuf is like a String, but for file systems paths (works cross-platform)
    path: std::path::PathBuf, 
}

fn main() {

    // the parse() method is provided by the clap::Parser trait
    // don't use this function outside of main()!
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
