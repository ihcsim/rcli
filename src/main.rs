use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    let args = Args::parse();
    let path = &args.path;
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file `{}`", path.display()))?;

    let mut buf = io::BufWriter::new(io::stdout());
    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(buf, "{}", line)?;
        }
    }
    Ok(())
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    path: PathBuf,
}

#[derive(Debug)]
struct CustomError(String);
