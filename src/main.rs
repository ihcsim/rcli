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
    let (lines_total, lines_matched) = find_matches(&pattern, &content, io::stdout(), &pb)?;
    pb.finish();

    println!(
        "lines match: ({}/{}), time elapsed: {:?}",
        lines_matched,
        lines_total,
        pb.elapsed()
    );
    Ok(())
}

fn find_matches(
    pattern: &str,
    content: &str,
    mut w: impl Write,
    pb: &ProgressBar,
) -> Result<(i32, i32)> {
    let mut lines_total = 0;
    let mut lines_matched = 0;
    for line in content.lines() {
        lines_total += 1;
        if line.contains(pattern) {
            lines_matched += 1;
            writeln!(w, "{}", line)?;
        }
        pb.inc(1);
    }
    Ok((lines_total, lines_matched))
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

#[test]
fn test_find_matches() -> Result<()> {
    let mut result = Vec::new();
    let pb = ProgressBar::new(100);
    let (lines_total, lines_matched) =
        find_matches("lorem", "lorem ipsum\ndolor sit amet", &mut result, &pb)?;
    pb.finish();

    assert_eq!(result, b"lorem ipsum\n");
    assert_eq!(lines_total, 2);
    assert_eq!(lines_matched, 1);
    Ok(())
}
