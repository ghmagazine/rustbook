use cli_test_dir::*;

// バイナリを実行して入出力を確認するテスト
// `cargo test` で実行できる

#[test]
fn run_benchmark() {
    let testdir = TestDir::new("./examples/benchmark", "Run benchmark");
    let output = testdir
        .cmd()
        .arg("20")
        // .tee_output()    // stdoutとstderrorを順に表示する。デバッグに便利
        .expect_success();

    use regex::Regex;
    let re = Regex::new(
        r#"(?m)\Asorting 1048576 integers \(4.0 MB\)
cpu info: \d+ physical cores, \d+ logical cores
seq_sort: sorted 1048576 integers in \d+\.\d+ seconds
par_sort: sorted 1048576 integers in \d+\.\d+ seconds
speed up: \d+\.\d+x
\z"#,
    )
    .expect("Failed to compile a regex pattern");

    assert!(re.is_match(output.stdout_str()));
    assert!(output.stderr_str().is_empty());
}
