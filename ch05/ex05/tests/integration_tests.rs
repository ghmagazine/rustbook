use cli_test_dir::*;

// バイナリを実行して入出力を確認するテスト
// `cargo test` で実行できる

#[test]
fn run_ch05_01() {
    let testdir = TestDir::new("./examples/ch05_01_box", "Run ch05_01");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_02() {
    let testdir = TestDir::new("./examples/ch05_02_vec", "Run ch05_02");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_03() {
    let testdir = TestDir::new("./examples/ch05_03_boxed_slice", "Run ch05_03");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"v1 len: 5, capacity: 8
v1 len: 5, capacity: 5
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_04() {
    let testdir = TestDir::new("./examples/ch05_04_hash_map", "Run ch05_04");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_05() {
    let testdir = TestDir::new("./examples/ch05_05_string1", "Run ch05_05");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_06() {
    let testdir = TestDir::new("./examples/ch05_06_string2", "Run ch05_06");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        "Err(ParseFloatError { kind: Invalid })\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_07() {
    let testdir = TestDir::new("./examples/ch05_07_string3", "Run ch05_07");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_08() {
    let testdir = TestDir::new("./examples/ch05_08_string4", "Run ch05_08");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_09() {
    let testdir = TestDir::new("./examples/ch05_09_range", "Run ch05_09");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_10() {
    let testdir = TestDir::new("./examples/ch05_10_option", "Run ch05_10");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_11() {
    let testdir = TestDir::new("./examples/ch05_11_result", "Run ch05_11");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        "Err(ParseIntError { kind: InvalidDigit })\n"
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_12() {
    let testdir = TestDir::new("./examples/ch05_12_type_alias", "Run ch05_12");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_13() {
    let testdir = TestDir::new("./examples/ch05_13_struct1", "Run ch05_13");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_14() {
    let testdir = TestDir::new("./examples/ch05_14_struct2", "Run ch05_14");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_15() {
    let testdir = TestDir::new("./examples/ch05_15_struct3", "Run ch05_15");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_16() {
    let testdir = TestDir::new("./examples/ch05_16_struct4", "Run ch16_??");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_17() {
    let testdir = TestDir::new("./examples/ch05_17_struct5", "Run ch05_17");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_18() {
    let testdir = TestDir::new("./examples/ch05_18_enum1", "Run ch05_18");
    let output = testdir.cmd().expect_success();
    assert_eq!(output.stdout_str(), "TGIF!\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_19() {
    let testdir = TestDir::new("./examples/ch05_19_enum2", "Run ch05_19");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"タスク0はjunkoさんにアサインされています
タスク1はhiroさんが作業中です。残り18時間の見込み
タスク2はその他のステータス（Done）です
"#,
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_20() {
    let testdir = TestDir::new("./examples/ch05_20_adv_types1", "Run ch05_20");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_21() {
    let testdir = TestDir::new("./examples/ch05_21_adv_types2", "Run ch05_21");
    let output = testdir.cmd().expect_success();

    use regex::Regex;
    let re = Regex::new(
        r#"(?m)\Astruct A \(\d{1,2} bytes\)
  f0: 0x[0-9a-f]+
  f1: 0x[0-9a-f]+
  f2: 0x[0-9a-f]+

\z"#,
    )
    .expect("Failed to compile a regex pattern");

    assert!(re.is_match(output.stdout_str()));
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_22() {
    let testdir = TestDir::new("./examples/ch05_22_type_cast1", "Run ch05_22");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_23() {
    let testdir = TestDir::new("./examples/ch05_23_type_cast2", "Run ch05_23");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_24() {
    let testdir = TestDir::new("./examples/ch05_24_transmute", "Run ch05_24");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"5678
1169258291
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_25() {
    let testdir = TestDir::new("./examples/ch05_25_type_coercion1", "Run ch05_25");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_26() {
    let testdir = TestDir::new("./examples/ch05_26_type_coercion2", "Run ch05_26");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_27() {
    let testdir = TestDir::new("./examples/ch05_27_type_coercion3", "Run ch05_27");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_28() {
    let testdir = TestDir::new("./examples/ch05_28_type_coercion4", "Run ch05_28");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch05_29() {
    let testdir = TestDir::new("./examples/ch05_29_type_coercion5", "Run ch05_29");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}
