use std::env;
use std::path::PathBuf;

fn main() {
    // onigmoの共有ライブラリを使うことをcargoがrustcに伝えるように伝える
    println!("cargo:rustc-link-lib=onigmo");

    // `binden::Builder`がbindgenを使うときのメインのエントリーポイント
    // オプションを設定できます。
    let bindings = bindgen::Builder::default()
        // バインディングを作る基になるヘッダファイル
        .header("wrapper.h")
        // `bindgen_test_layout_max_align_t` のテスト失敗を防ぐためにRust 1.28.0以上向けにコード生成する
        .rust_target(bindgen::RustTarget::Stable_1_28)
        // ビルダーを完了してバインディングを生成する
        .generate()
        .expect("Unable to generate bindings");

    // バインディングを`$OUT_DIR/bindings.rs`に書き出す。
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
