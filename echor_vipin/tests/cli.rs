use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
type TestResult = Result<(), Box<dyn std::error::Error>>;


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
fn hello1() {
let outfile = "tests/expected/hello1.txt";
let expected = fs::read_to_string(outfile).expect("inadequate string file");
let mut cmd = Command::cargo_bin("echor_vipin")
.unwrap();
println!("{:?}",expected);
cmd.arg("Hello there")
.arg("-n")
.assert()
.success()
.stdout(expected);
}

#[test]
fn hello2() {
let outfile = "tests/expected/hello2.txt";
let expected = fs::read_to_string(outfile).expect("inadequate string file");
let mut cmd = Command::cargo_bin("echor_vipin")
.unwrap();
cmd.arg("Hello")
.arg("there")
.arg("-n")
.assert()
.success()
.stdout(expected);
}