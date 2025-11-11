use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::prelude::*;

#[test]
fn help() {
    let cmd = cargo_bin_cmd!().unwrap();
    cmd.assert().stdout("Hello, world!\n").success();
}
