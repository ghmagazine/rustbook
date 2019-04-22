use cli_test_dir::*;

// examplesディレクトリ配下にあるバイナリを実行して入出力を確認するテスト
// `cargo test` で実行できる

#[test]
fn run_ch07_01() {
    let testdir = TestDir::new("./examples/ch07_01_value_scope", "Run ch07_01");
    let output = testdir
        .cmd()
        //.tee_output()    // stdoutとstderrorを順に表示する。デバッグに便利
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"(a)  p1: Parent(1, Child(11), Child(12)), p2: Parent(2, Child(21), Child(22))
Dropping Parent(2, Child(21), Child(22))
Dropping Child(21)
Dropping Child(22)
(b)  p1: Parent(1, Child(11), Child(12))
(c)  p1: Parent(1, Child(11), Child(12)), p3: Parent(3, Child(31), Child(32))
Dropping Parent(3, Child(31), Child(32))
Dropping Child(31)
Dropping Child(32)
Dropping Parent(1, Child(11), Child(12))
Dropping Child(11)
Dropping Child(12)
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch07_02() {
    let testdir = TestDir::new("./examples/ch07_02_move_semantics", "Run ch07_02");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"Move Semantics:
p2: Parent(1, Child(11), Child(12))
p1: Parent(2, Child(21), Child(22))
Dropping Parent(1, Child(11), Child(12))
Dropping Child(11)
Dropping Child(12)
Dropping Parent(2, Child(21), Child(22))
Dropping Child(21)
Dropping Child(22)

Borrow:
p:  Parent(1, Child(11), Child(12))
p1: Parent(1, Child(11), Child(12))
Dropping Parent(1, Child(11), Child(12))
Dropping Child(11)
Dropping Child(12)
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch07_03() {
    let testdir = TestDir::new("./examples/ch07_03_nll", "Run ch07_03");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch07_04() {
    let testdir = TestDir::new("./examples/ch07_04_static_lifetime", "Run ch07_04");
    let output = testdir.cmd().expect_success();
    assert_eq!(output.stdout_str(), "42\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch07_05() {
    let testdir = TestDir::new("./examples/ch07_05_rc", "Run ch07_05");
    let output = testdir.cmd().expect_success();
    assert_eq!(
        output.stdout_str(),
        r#"(a) count: 1, rc1: Child(1)
(b) count: 2, rc1: Child(1), rc2: Child(1)
(c) count: 1, rc1: Child(1)
(d) count: 1, rc1: Child(2)
(e) count: 1, rc1: Child(2), weak: (Weak)
(f) count: 2, rc1: Child(2), rc3: Child(2)
Dropping Child(2)
(g) count: 0, weak.upgrade(): None
"#
    );
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch07_06() {
    let testdir = TestDir::new("./examples/ch07_06_simple_refcell", "Run ch07_06");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch07_07() {
    let testdir = TestDir::new("./examples/ch07_07_tls_refcell", "Run ch07_07");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch07_08() {
    let testdir = TestDir::new("./examples/ch07_08_arc_rwlock", "Run ch07_08");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch07_09() {
    let testdir = TestDir::new("./examples/ch07_09_static_rwlock", "Run ch07_09");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}

#[test]
fn run_ch07_10() {
    let testdir = TestDir::new("./examples/ch07_10_closure", "Run ch07_10");
    let output = testdir.cmd().expect_success();
    assert!(output.stdout_str().is_empty());
    assert!(output.stderr_str().is_empty());
}
