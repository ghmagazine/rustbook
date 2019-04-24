# 『実践Rust入門』のサンプルプログラム

**実践Rust入門**</br>
[言語仕様から開発手法まで]</br></br>
κeen(著)、河野達也(著)、小松礼人(著)</br>
B5判／576ページ／本体価格3,980円＋税</br>
ISBN番号 978-4-297-10559-4</br>
技術評論社、2019年5月8日発行

## サンプルプログラム

本GitHubリポジトリでは『**実践Rust入門**』（以降 **本書**）に掲載されているサンプルプログラムを収録しています。
本書の各章に対応する`ch章番号`ディレクトリを参照してください。

### ダウンロード方法

[リリースページ][releases-page] にある`*.zip`または`*.tar.gz`のアーカイブファイルをダウンロード後、解凍してください。
どちらのアーカイブも内容は同じです。
WindowsとmacOSをお使いなら`*.zip`を、Linuxをお使いなら`*.tar.gz`をダウンロードすることをお勧めします。

それぞれのリリースの内容については [RELEASES.mdファイル][releases-md] を参照してください。

[releases-page]: https://github.com/ghmagazine/rustbook/releases
[releases-md]: ./RELEASES.md

### 改行文字について

ソースファイルの改行文字はLinuxやmacOS環境で使用されている`LF`文字になります。Windows環境ではアプリケーションによっては正しく改行されないかもしれません。2章を参考に、Visual Studio Code（VS Code）などのソースコードエディタを使われることをお勧めします。

## 本書で使用する追加ソフトウェアについて

### インストール方法

本書で使用する追加ソフトウェアのインストール方法については以下を参考にしてください。

- [DockerとDocker Composeのインストール][docker]（11章で使用）
- Windows MSVC：[vcpkgパッケージマネージャのインストール][vcpkg]（11章で使用）

[docker]: ./install/docker.md
[vcpkg]: ./install/windows10-vcpkg.md

なお、Rustツールチェインとリンカのインストール方法については本書の2章を参照してください。

### 使いかたなど

追加ソフトウェアの使いかたについては以下を参考にしてください。

- Windows MSVC：[C/C++コンパイラをコマンドプロンプトから実行する方法][msvc-compiler]（12章で使用）

[msvc-compiler]: ./howto/running-msvc-compiler.md

## 動作確認の環境について

サンプルプログラムは以下のRust/OSバージョンにて動作確認済みです。

- Rust 1.32.0（2019年1月16日リリース）
  * 2018 Editionを指定
- x86-64系のプロセッサで動作する以下のOS
  * Ubuntu "Bionic" 18.04 LTS（64ビット）
  * macOS Mojave 10.14
  * Windows 10 （64ビット、Microsoft Visual C++ 2017）

## ご質問や不具合報告など

本書やサンプルプログラムの内容についてご質問などあるときは、以下の方法でご確認・ご連絡ください。

- [本書の公式サポートページの正誤表][errata] で報告・訂正されていないかご確認
- [本書の公式サポートページのお問い合わせフォーム][inquiry-form] で編集部へお問い合わせ
- Slack **rust-jp**チームの `#rust-bicycle-book` チャネルで著者らに質問
  * 参加登録URL： http://rust-jp.herokuapp.com/
- サンプルプログラムの明らかなバグなら、本リポジトリの [issueページ][gh-issues] で直接ご報告いただいても構いません。（その場合でもSlackなどでご一報いただいてからの方がスムーズに対応できるかもしれません）

<!-- 現時点で正誤表が公開されていないため、公開されるまでは書籍紹介ページへリンクしておく -->
[errata]: https://gihyo.jp/book/2019/978-4-297-10559-4
<!-- [errata]: https://gihyo.jp/book/2019/978-4-297-10559-4/support -->

[inquiry-form]: https://gihyo.jp/site/inquiry/book?ISBN=978-4-297-10559-4
[gh-issues]: https://github.com/ghmagazine/rustbook/issues

## 本書の公式サポートページ

https://gihyo.jp/book/2019/978-4-297-10559-4

本書の概要や目次、正誤表などが掲載されています。

## ライセンス

本リポジトリのコンテンツは特に断り書きがない限り **三条項BSDライセンス** のもとで公開されています。

- https://github.com/ghmagazine/rustbook/blob/master/LICENSE

以下のディレクトリ配下のコンテンツは異なるライセンスで公開されています。

- `ch10/wordcount`： **MIT** と **Apache-2.0** のデュアルライセンス
