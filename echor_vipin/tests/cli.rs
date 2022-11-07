use assert_cmd::Command;
use predicates::prelude::*;

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