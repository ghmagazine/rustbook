use cli_test_dir::*;

// バイナリを実行して入出力を確認するテスト
// `cargo test` で実行できる

#[test]
fn run_ch04_01() {
    let testdir = TestDir::new("./examples/ch04_01_unit", "Run ch04_01");
    let output = testdir.cmd().expect_success();
    assert_eq!(output.stdout_str(), "Hello\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_02() {
    let testdir = TestDir::new("./examples/ch04_02_bool", "Run ch04_02");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_03() {
    let testdir = TestDir::new("./examples/ch04_03_integer", "Run ch04_03");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_04() {
    let testdir = TestDir::new("./examples/ch04_04_overflowed1", "Run ch04_04");
    let output = testdir
        .cmd()
        // .tee_output()
        .expect_failure(); // debugビルドではpanicする
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().contains("thread 'main' panicked at 'attempt to add with overflow', examples/ch04_04_overflowed1.rs:"));
}

#[test]
fn run_ch04_05() {
    let testdir = TestDir::new("./examples/ch04_05_overflowed2", "Run ch04_05");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_06() {
    let testdir = TestDir::new("./examples/ch04_06_float", "Run ch04_06");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_07() {
    let testdir = TestDir::new("./examples/ch04_07_char", "Run ch04_07");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_08() {
    let testdir = TestDir::new("./examples/ch04_08_reference1", "Run ch04_08");
    let output = testdir.cmd().expect_success();

    use regex::Regex;
    let re = Regex::new(
        r#"(?m)\Amain:     n = 0
f1:       n = 1
main:     n = 0
f2:   n_ptr = 0x[0-9a-f]+
f2:  \*n_ptr = 2
main:     n = 2
\z"#,
    )
    .expect("Failed to compile a regex pattern");

    assert!(re.is_match(output.stdout_str()));
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_09() {
    let testdir = TestDir::new("./examples/ch04_09_reference2", "Run ch04_09");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_10() {
    let testdir = TestDir::new("./examples/ch04_10_raw_pointer", "Run ch04_10");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_11() {
    let testdir = TestDir::new("./examples/ch04_11_fn_pointer", "Run ch04_11");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_12() {
    let testdir = TestDir::new("./examples/ch04_12_fn_pointer_vs_closure", "Run ch04_12");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_13() {
    let testdir = TestDir::new("./examples/ch04_13_tuple", "Run ch04_13");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_14() {
    let testdir = TestDir::new("./examples/ch04_14_array", "Run ch04_14");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        "a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,a,",
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_15() {
    let testdir = TestDir::new("./examples/ch04_15_slice1", "Run ch04_15");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"a1: ['a', 'b', 'c', 'd']
  &a1[..]   - 4, Some('a'), 'b', Some('d')
  &a1       - 4, Some('a'), 'b', Some('d')
  &a1[1..3] - 2, Some('b'), 'c', Some('c')

v1: ['e', 'f', 'g', 'h']
  &v1[..]   - 4, Some('e'), 'f', Some('h')
  &v1       - 4, Some('e'), 'f', Some('h')
  &v1[1..3] - 2, Some('f'), 'g', Some('g')
"#
    );
    assert!(output.stderr_str().is_empty());
}

/*


*/

#[test]
fn run_ch04_16() {
    let testdir = TestDir::new("./examples/ch04_16_slice2", "Run ch04_16");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch04_17() {
    let testdir = TestDir::new("./examples/ch04_17_str", "Run ch04_17");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        "かか\u{3099}く\nErr(Utf8Error { valid_up_to: 0, error_len: Some(1) })\n",
    );
    assert!(output.stderr_str().is_empty());
}
