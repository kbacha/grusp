extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;

    #[test]
    fn it_can_find_fixture() {
        assert_cli::Assert::main_binary()
            .with_args(&["FIND THIS", "./tests/fixtures"])
            .succeeds()
            .stdout()
            .contains("FIND THIS")
            .unwrap();
    }

    #[test]
    fn it_can_find_this_ignoring_case() {
        assert_cli::Assert::main_binary()
            .with_args(&["--ignore-case", "FiNd ThIs", "./tests/fixtures"])
            .succeeds()
            .stdout()
            .contains("FIND THIS")
            .unwrap();
    }

    #[test]
    fn it_can_find_this_case_sensitively() {
        assert_cli::Assert::main_binary()
            .with_args(&["--case-sensitive", "FiNd ThIs", "./tests/fixtures"])
            .fails_with(1)
            .stdout()
            .not()
            .contains("FIND THIS")
            .unwrap();
    }

    #[test]
    fn it_exits_with_status_one_when_finding_nothing() {
        assert_cli::Assert::main_binary()
            .with_args(&["find nothing at all", "./tests/fixtures"])
            .fails_with(1)
            .unwrap();
    }

    #[test]
    fn it_will_search_multiple_files() {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "--ignore-case",
                    "--nocolor",
                    "find",
                    "./tests/fixtures/example-1.txt",
                    "./tests/fixtures/example-2.txt",
                ],
            )
            .succeeds()
            .stdout()
            .contains("find")
            .stdout()
            .contains("FIND THIS")
            .unwrap();
    }

    #[test]
    fn it_will_count_matches() {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "--ignore-case",
                    "--nocolor",
                    "--count",
                    "find",
                    "./tests/fixtures/example-1.txt",
                    "./tests/fixtures/example-2.txt",
                ],
            )
            .succeeds()
            .stdout()
            .contains("example-1.txt matched 1 time")
            .stdout()
            .not()
            .contains("find")
            .unwrap();
    }

    #[test]
    fn it_can_find_files_with_matches() {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "--nocolor",
                    "--files-with-matches",
                    "FIND",
                    "./tests/fixtures/example-1.txt",
                    "./tests/fixtures/example-2.txt",
                ],
            )
            .succeeds()
            .stdout()
            .contains("example-1.txt")
            .stdout()
            .not()
            .contains("example-2.txt")
            .unwrap();
    }

    #[test]
    fn it_can_find_files_without_matches() {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "--nocolor",
                    "--files-without-matches",
                    "FIND",
                    "./tests/fixtures/example-1.txt",
                    "./tests/fixtures/example-2.txt",
                ],
            )
            .succeeds()
            .stdout()
            .contains("example-2.txt")
            .stdout()
            .not()
            .contains("example-1.txt")
            .unwrap();
    }

    #[test]
    fn succeeds_when_finding_files_without_matches_and_none_match() {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "--nocolor",
                    "--files-without-matches",
                    "gibberish",
                    "./tests/fixtures/example-1.txt",
                    "./tests/fixtures/example-2.txt",
                ],
            )
            .succeeds()
            .stdout()
            .contains("example-2.txt")
            .stdout()
            .contains("example-1.txt")
            .unwrap();
    }
}
