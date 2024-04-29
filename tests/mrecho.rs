#[cfg(test)]
mod rmecho {
    use assert_cmd::Command;
    use predicates::str::contains;

    const PROG_NAME: &str = env!("CARGO_PKG_NAME");

    #[test]
    fn respects_no_newline() {
        Command::cargo_bin(PROG_NAME)
            .unwrap()
            .args(["-n", "hello", "world"])
            .assert()
            .stdout(contains("hello world"));
    }

    #[test]
    fn runs() {
        Command::cargo_bin(PROG_NAME)
            .unwrap()
            .args(["hello", "world"])
            .assert()
            .stdout(contains("hello world\n"));
    }

    #[test]
    fn dies_with_no_args() {
        Command::cargo_bin(PROG_NAME)
            .unwrap()
            .assert()
            .failure()
            .stderr(contains("Usage"));
    }

    #[test]
    fn has_version() {
        Command::cargo_bin(PROG_NAME)
            .unwrap()
            .args(["--version"])
            .assert()
            .stdout(contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn has_help() {
        Command::cargo_bin(PROG_NAME)
            .unwrap()
            .args(["--help"])
            .assert()
            .stdout(contains("Usage: ".to_owned() + PROG_NAME));
    }
}
