use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

#[test]
fn cli_no_args() {
    // Test that `kvs` with no args should exit with a non-zero code.
    Command::cargo_bin("kvs").unwrap().assert().failure();
}

#[test]
fn cli_version() {
    // Test that `kvs -V` should print the version.
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_get() {
    // Test that `kvs get <KEY>` should print "unimplemented" to stderr and exit with non-zero code.
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["get", "key1"])
        .assert()
        .failure()
        .stderr(contains("unimplemented"));
}

#[test]
fn cli_set() {
    // Test that `kvs set <KEY> <VALUE>` should print "unimplemented" to stderr and exit with non-zero code.
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["set", "key1", "value1"])
        .assert()
        .failure()
        .stderr(contains("unimplemented"));
}

#[test]
fn cli_rm() {
    // Test that `kvs rm <KEY>` should print "unimplemented" to stderr and exit with non-zero code.
    Command::cargo_bin("kvs")
        .unwrap()
        .args(&["rm", "key1"])
        .assert()
        .failure()
        .stderr(contains("unimplemented"));
}

// TODO:
// 1. invalid get
// 2. invalid set
// 3. invalid remove
// 4. invalid subcommand
// 5. get value
// 6. overwrite value
// 7. get non existent value
// 8. remove key
