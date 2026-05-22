use anyhow::{Context, Result};
use clap::Parser;
use crossbeam_channel::{Receiver, Sender, bounded};
use human_panic::setup_panic;
use indicatif::ProgressBar;
use std::io::Write;
use std::thread;
use std::{fs, path::PathBuf, time::Duration};

fn main() -> Result<()> {
    setup_panic!();

    let (sender, receiver): (Sender<()>, Receiver<()>) = bounded(2);
    let t = thread::spawn(move || {
        if let Err(e) = ctrlc::set_handler(move || {
            let _ = sender.send(());
        }) {
            eprintln!("error setting Ctrl-C handler: {}", e);
            return;
        }
        thread::park();
    });

    let args = Args::parse();
    let pattern = args.pattern.clone();
    let path = &args.path;
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read file `{}`", path.display()))?;
    let pb = ProgressBar::new(100);
    let mut buf = Vec::new();
    let (lines_total, lines_matched) = find_matches(&pattern, &content, &mut buf, receiver, &pb)?;
    pb.finish();

    if args.json {
        json_output(
            buf,
            &pattern,
            path.to_str().unwrap_or_default(),
            lines_total,
            lines_matched,
            pb.elapsed(),
        )?;
    } else {
        stdout(buf, lines_total, lines_matched, pb.elapsed())?;
    }

    t.thread().unpark();
    t.join()
        .map_err(|e| anyhow::anyhow!("failed to join Ctrl-C handler thread: {:?}", e))
        .with_context(|| "failed to join Ctrl-C handler thread")?;
    Ok(())
}

fn find_matches(
    pattern: &str,
    content: &str,
    mut w: impl Write,
    receiver: Receiver<()>,
    pb: &ProgressBar,
) -> Result<(i32, i32)> {
    let mut lines_total = 0;
    let mut lines_matched = 0;
    for line in content.lines() {
        if receiver.try_recv().is_ok() {
            println!("received Ctrl-C, exiting...");
            break;
        };

        lines_total += 1;
        if line.contains(pattern) {
            lines_matched += 1;
            writeln!(w, "{}", line)?;
        }
        pb.inc(1);
    }
    Ok((lines_total, lines_matched))
}

fn stdout(buf: Vec<u8>, lines_total: i32, lines_matched: i32, elapsed: Duration) -> Result<()> {
    for line in String::from_utf8(buf)
        .with_context(|| "failed to convert to string")?
        .lines()
    {
        println!("{}", line);
    }
    println!(
        "lines match: ({}/{}), time elapsed: {:?}",
        lines_matched, lines_total, elapsed
    );
    Ok(())
}

fn json_output(
    buf: Vec<u8>,
    pattern: &str,
    path: &str,
    lines_total: i32,
    lines_matched: i32,
    elapsed: Duration,
) -> Result<()> {
    let buf_str = String::from_utf8(buf).with_context(|| "failed to convert to string")?;
    let matches: Vec<&str> = buf_str.split('\n').filter(|x| !x.is_empty()).collect();
    let result = serde_json::json!({
        "pattern": pattern,
        "path": path,
        "lines_total": lines_total,
        "lines_matched": lines_matched,
        "time_elapsed": elapsed,
        "matches": matches,
    });
    println!("{}", serde_json::to_string_pretty(&result)?);
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

    /// Output in JSON format
    #[clap(short, long, default_value_t = false)]
    json: bool,
}

#[test]
fn test_find_matches() -> Result<()> {
    let mut result = Vec::new();
    let pb = ProgressBar::new(100);
    let (_, receiver) = bounded(2);
    let (lines_total, lines_matched) = find_matches(
        "lorem",
        "lorem ipsum\ndolor sit amet",
        &mut result,
        receiver,
        &pb,
    )?;
    pb.finish();

    assert_eq!(result, b"lorem ipsum\n");
    assert_eq!(lines_total, 2);
    assert_eq!(lines_matched, 1);
    Ok(())
}
