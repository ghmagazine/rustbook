# Windows MSVC：vcpkgパッケージマネージャのインストール

vcpkgはC++ライブラリを管理するためのパッケージマネージャで、Microsoftがオープンソースで提供しています。Windows、Linux、macOSに対応しています。

本書ではWindows MSVC環境をご使用の場合のみvcpkgが必要になります。具体的には11章でlibpqというPostgreSQLのクライアントライブラリをインストールするためにvcpkgを使います。ここではWindows MSVC環境にvcpkgをインストールする手順を説明します。

## 準備

WindowsでvcpkgをインストールするにはMSVCのC++コンパイラが必要です。
本書の2-1-7項を参照して、Visual C++ Build Tools、または、Visual Studio Communityを事前にインストールしてください。

## vcpkgのソースコードのダウンロード

GitHubからvcpkgのソースコードを取得します。
ウェブブラウザで以下のURLを開いてください。
vcpkgリポジトリの最新の内容がzipファイルとしてダウンロードされます。

- https://github.com/Microsoft/vcpkg/archive/master.zip

このzipファイルをインストールしたい場所（例：`C:\src\vcpkg`）へ展開します。

## vcpkgのビルド

PowerShellかコマンドプロンプトでvcpkgを展開したディレクトリへ移動します。
以下のように`bootstrap-vcpkg.bat`を実行し、vcpkgをビルドします。

```powershell
# vcpkgを展開したディレクトリへ移動する
PS> cd c:\src\vcpkg

# vcpkgをビルドする
PS> .\bootstrap-vcpkg.bat
...
Building vcpkg.exe... done.

# ビルドできたのでバージョンを表示する
PS> .\vcpkg version
Vcpkg package management program version 2018.11.23-nohash

See LICENSE.txt for license information.
```

## 環境変数を設定する

環境変数`VCPKG_ROOT`にvcpkgのインストールディレクトリを設定してください。
PowerShellで一時的に設定するなら以下のようにします。

```powershell
PS> $Env:VCPKG_ROOT += "c:\src\vcpkg"
```

この設定を忘れると11章の`diesel_cli`や`log_collector`のビルドに失敗しますので注意してください。

これでvcpkgのインストールは完了です。

## vcpkgの詳しい情報

本書11章のプログラムを実行するだけでしたら、vcpkg自体については特に知らなくても問題ありません。11章に書かれているとおりコマンドを実行すれば大丈夫です。

もしvcpkgについて詳しく知りたくなったら、以下の日本語ドキュメントが役に立つでしょう。

- https://docs.microsoft.com/ja-jp/cpp/build/vcpkg?view=vs-2019
