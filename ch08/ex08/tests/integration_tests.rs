use cli_test_dir::{ExpectStatus, OutputExt, TestDir};

// examplesディレクトリ配下にあるバイナリを実行して入出力を確認するテスト
// `cargo test` で実行できる

#[test]
fn run_ch08_01() {
    // use cli_test_dir::TeeOutputExt;
    let testdir = TestDir::new("./examples/ch08_01_trait_basics", "Run ch08_01");
    let output = testdir
        .cmd()
        // .tee_output()    // stdoutとstderrorを順に表示する。デバッグに便利
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"x = 1, y = 1
r = 1.4142135623730951, θ = 0.7853981633974483
"#,
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_02() {
    let testdir = TestDir::new("./examples/ch08_02_trait_basics", "Run ch08_02");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"(0, 1)
(0.00000000000000006123233995736766, 1)
"#,
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_03() {
    let testdir = TestDir::new("./examples/ch08_03_trait_basics", "Run ch08_03");
    let output = testdir.cmd().expect_success();
    assert_eq!(output.stdout_str(), "(-1, 0.00000000000000012246467991473532)\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_04() {
    let testdir = TestDir::new("./examples/ch08_04_trait_generics", "Run ch08_04");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_05() {
    let testdir = TestDir::new("./examples/ch08_05_trait_generics", "Run ch08_05");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_06() {
    let testdir = TestDir::new("./examples/ch08_06_overload", "Run ch08_06");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_07() {
    let testdir = TestDir::new("./examples/ch08_07_trait_object", "Run ch08_07");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_08() {
    let testdir = TestDir::new("./examples/ch08_08_existential_impl_trait", "Run ch08_08");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_09() {
    let testdir = TestDir::new("./examples/ch08_09_associated_const", "Run ch08_09");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_10() {
    let testdir = TestDir::new("./examples/ch08_10_associated_type", "Run ch08_10");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_11() {
    let testdir = TestDir::new("./examples/ch08_11_sized", "Run ch08_11");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(), 
        r#"8
16
"#,
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch08_12() {
    let testdir = TestDir::new("./examples/ch08_12_trait_techniques", "Run ch08_12");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(), 
        r#"true
1
"#,
    );
    assert!(output.stderr_str().is_empty());
}
