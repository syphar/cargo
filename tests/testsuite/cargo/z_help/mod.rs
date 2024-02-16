use cargo_test_support::file;
use cargo_test_support::prelude::*;

#[cargo_test]
fn case() {
    snapbox::cmd::Command::cargo_ui()
        .masquerade_as_nightly_cargo(&["-Z help"])
        .args(["-Z", "help"])
        .assert()
        .success()
        .stdout_matches(file!["stdout.log"])
        .stderr_matches(file!["stderr.log"]);
}