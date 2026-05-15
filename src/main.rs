use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use std::io::{self, Write};
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    let args = Args::parse();
    let pattern = args.pattern.clone();
    let path = &args.path;
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file `{}`", path.display()))?;

    let pb = ProgressBar::new(100);
    let mut buf = io::BufWriter::new(io::stdout());
    let mut lines_total = 0;
    let mut lines_matched = 0;
    for line in content.lines() {
        lines_total += 1;
        if line.contains(&args.pattern) {
            lines_matched += 1;
            writeln!(buf, "{}", line)?;
        }
        pb.inc(1);
    }
    pb.finish();

    buf.flush()?;
    println!(
        "lines match: ({}/{}), time elapsed: {:?}",
        lines_matched,
        lines_total,
        pb.elapsed()
    );
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
