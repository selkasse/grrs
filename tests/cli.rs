use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    // Point Command to our main binary, as "grrs" is the package name
    let mut cmd = Command::cargo_bin("grrs")?;

    // Run `grrs foobar test/file/doesnt/exist`
    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn no_match_found() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("blorp").arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::is_empty());

    Ok(())
}

#[test]
fn empty_string_as_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A space\nThis is space test\nMore space\nAnother space")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("pattern must not be empty"));

    Ok(())
}

#[test]
fn missing_pattern() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("This\nWon't\nMatter\nAnother space")?;

    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg(file.path());

    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided",
    ));

    Ok(())
}

#[test]
fn missing_path() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("This\nWon't\nMatter\nAnother space")?;

    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("testpattern");

    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided",
    ));

    Ok(())
}

#[test]
fn missing_pattern_and_path() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("This\nWon't\nMatter\nAnother space")?;

    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided",
    ));

    Ok(())
}
