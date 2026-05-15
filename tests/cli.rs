use assert_cmd::cargo::*;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn file_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("rcli");
    cmd.arg("foobar").arg("test/file/not_found");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read file"));
    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = cargo_bin_cmd!("rcli");
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));
    file.close().unwrap();
    Ok(())
}
