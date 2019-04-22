use cli_test_dir::*;

// バイナリを実行して入出力を確認するテスト
// `cargo test` で実行できる

const CMD: &'static str = "./examples/println";

#[test]
fn verify_output() {
    let testdir = TestDir::new(CMD, "Verify output");
    let output = testdir
        .cmd()
        // .tee_output()   // stdoutとstderrorを順に表示する。デバッグに便利
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"Hello, Takashi!
半径 3.2、円周率 3.142、面積 32.170
"#
    );
}
