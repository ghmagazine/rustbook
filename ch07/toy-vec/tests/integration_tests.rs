use cli_test_dir::*;

// バイナリを実行して入出力を確認するテスト
// `cargo test` で実行できる

#[test]
fn run_toy_vec_01() {
    let testdir = TestDir::new("./examples/toy_vec_01", "Run ToyVec example 01");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_toy_vec_02() {
    let testdir = TestDir::new("./examples/toy_vec_02", "Run ToyVec example 02");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_toy_vec_03() {
    let testdir = TestDir::new("./examples/toy_vec_03", "Run ToyVec example 03");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}
