use cli_test_dir::*;

// バイナリ（leap-year）を実行して入出力を確認するテスト
// `cargo test` で実行できる

const CMD: &'static str = "./leap-year";

#[test]
fn year_2000_is_leap_year() {
    let testdir = TestDir::new(CMD, "Test year 2000");
    let output = testdir
        .cmd()
        .output_with_stdin("2000\n")
        // .tee_output()   // stdoutとstderrorを順に表示する。デバッグに便利
        .expect_success();
    assert!(output.stdout_str().contains("2000 is a leap year!"));
}

#[test]
fn year_2019_is_not_leap_year() {
    let testdir = TestDir::new(CMD, "Test year 2019");
    let output = testdir.cmd().output_with_stdin("2019\n").expect_success();
    assert!(output.stdout_str().contains("2019 is not a leap year."));
}

#[test]
fn year_2020_is_leap_year() {
    let testdir = TestDir::new(CMD, "Test year 2020");
    let output = testdir.cmd().output_with_stdin("2020\n").expect_success();
    assert!(output.stdout_str().contains("2020 is a leap year!"));
}

#[test]
fn year_2100_is_not_leap_year() {
    let testdir = TestDir::new(CMD, "Test year 2100");
    let output = testdir.cmd().output_with_stdin("2100\n").expect_success();
    assert!(output.stdout_str().contains("2100 is not a leap year."));
}
