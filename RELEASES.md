# 1.0.0 (2019-04-24)

* サンプルプログラムの初版を公開
* サンプルプログラムの内容は以下を除いて書籍の**初版**に掲載されているものと同等
  * `ch12/onigmo-rs/onigmo-sys/build.rs`：`bindgen_test_layout_max_align_t`のテスト失敗を防ぐためにRust 1.28.0以上向けにコードを生成するよう修正（[diff][v1-0-0-onigmo-sys-build-diff]）
  * 書籍2-2-7項に掲載されている`ch02/examples/bin/println.rs`のファイルパスを、他の章のファイルのネーミングルールに合わせて[`ch02/ex02/examples/println.rs`][v1-0-0-ex02-println]に変更

[v1-0-0-onigmo-sys-build-diff]: https://github.com/ghmagazine/rustbook/commit/9bebf5b7c2a5f9f8a74323aa9613808f2aa2897b
[v1-0-0-ex02-println]: ./ch02/ex02/examples/println.rs
