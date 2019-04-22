use cli_test_dir::*;

// バイナリを実行して入出力を確認するテスト
// `cargo test` で実行できる

#[test]
fn run_rpn() {
    let testdir = TestDir::new("./rpn", "Run rpn");
    let output = testdir
        .cmd()
        // .tee_output()    // stdoutとstderrorを順に表示する。デバッグに便利
        .expect_success();
    assert_eq!(output.stdout_str(), "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * - = 26.2840\n");
    assert!(output.stderr_str().is_empty());
}
