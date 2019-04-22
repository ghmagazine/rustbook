use cli_test_dir::*;

// バイナリを実行して入出力を確認するテスト
// `cargo test` で実行できる

#[test]
fn run_hello() {
    let testdir = TestDir::new("./hello", "Run hello");
    let output = testdir
        .cmd()
        // .tee_output()    // stdoutとstderrorを順に表示する。デバッグに便利
        .expect_success();
    assert_eq!(output.stdout_str(), "Hello, world!\n");
    assert!(output.stderr_str().is_empty());
}
