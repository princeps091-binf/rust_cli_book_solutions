use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult{

    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor_vipin")?
.args(args)
.assert()
.success()
.stdout(expected);
Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
let mut cmd = Command::cargo_bin("echor_vipin")?;
cmd.assert()
.failure()
.stderr(predicate::str::contains("Usage"));
Ok(())
}

#[test]
fn runs() {
let mut cmd = Command::cargo_bin("echor_vipin").unwrap();
cmd.arg("hello -n").assert().success();
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"],"tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello","there"],"tests/expected/hello2.txt")

}

#[test]
fn hello1n() -> TestResult {
    run(&["Hello there","-n"],"tests/expected/hello1.n.txt")
}

#[test]
fn hello2n() -> TestResult {
    run(&["Hello", "there","-n"],"tests/expected/hello2.n.txt")
}
