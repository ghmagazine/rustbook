# DockerとDocker Composeのインストール

**Docker**は軽量なコンテナ型のアプリケーション実行環境です。
本書11章でリレーショナルデータベースのPostgreSQLを動かすために使用します。
Dockerには無償のCommunity Edition（CE）と有償のEnterprise Edition（EE）があります。
本書の用途では無償のCEで十分でしょう。

**Docker Compose**は1台のDockerホストマシン（みなさんのPCやMac）で使用できるオーケストレーションツールで、複数のコンテナで構成されるサービスを管理する時に役立ちます。
無償で使用できます。

## Linux

LinuxではディストリビューションごとにDocker CEのインストール手順が異なります。
ここではUbuntu（x86_64）を例に説明します。
他のディストリビューションについては、Dockerの公式ドキュメントにあるDocker CEのインストール手順を参照してください。

- https://docs.docker.com/install/

### Dockerのパッケージリポジトリの登録

`apt`にDockerのパッケージリポジトリを登録しましょう。

まず必要なツールをインストールします。

```console
$ sudo apt update
$ sudo apt install \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg-agent \
    software-properties-common
```

Docker公式のGPGキーを登録します。

```console
$ curl -fsSL https://download.docker.com/linux/ubuntu/gpg | \
  sudo apt-key add -

$ sudo apt-key fingerprint 0EBFCD88
pub   rsa4096 2017-02-22 [SCEA]
      9DC8 5822 9FC7 DD38 854A  E2D8 8D81 803C 0EBF CD88
uid   [ unknown] Docker Release (CE deb) <docker@docker.com>
sub   rsa4096 2017-02-22 [S]
```

[Get Docker CE for Ubuntu](https://docs.docker.com/install/linux/docker-ce/ubuntu/) にDocker公式のGPGキーのfingerprintが記載されているので、そのfingerprintと自分の登録したGPGキーのfingerprintが同じものであることを確認してください。
なお上記のfingerprint（ `9DC8 5822 9FC7 DD38 854A  E2D8 8D81 803C 0EBF CD88` ）はこの文書を作成するときに筆者が確認したfingerprintであり、単なる例です。fingerprintを確認するときには必ず一次情報を参照しましょう。

リポジトリを登録します。

```console
$ sudo add-apt-repository \
   "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
   $(lsb_release -cs) \
   stable"
```

### Docker CEのインストール

Docker CEをインストールします。

```console
$ sudo apt update
$ sudo apt install docker-ce docker-ce-cli containerd.io
```

### ログインユーザを`docker`グループに追加する

必須ではありませんが、`docker`コマンドが`sudo`なしで実行できるように、ユーザを`docker`グループに追加しましょう。

```console
$ sudo usermod -a -G docker $USER
```

グループの変更を反映するためにログアウトし、ログインしなおしてください。

### Dockerのインストール結果の確認

`docker run`コマンドで実際にコンテナが実行できることを確認します。

```
$ docker --version
Docker version 18.09.5, build e8ff056

$ docker run hello-world
...
Hello from Docker!
This message shows that your installation appears to be working correctly.
...
```

### Docker Composeのインストール

続いてDocker Composeをインストールしましょう。
Webブラウザで以下のページを開きます。

- https://github.com/docker/compose/releases/latest

Docker Composeの最新バージョンのリリースページが表示されますので、バージョン（例：1.24.0）が確認できたら以下のコマンドを実行します。

```
$ sudo -i
# export VERSION='1.24.0'    # 最新リリースのバージョン
# export BASE_URL='https://github.com/docker/compose/releases/download'
# curl  -L ${BASE_URL}/${VERSION}/docker-compose-`uname -s`-`uname -m` \
        -o /usr/local/bin/docker-compose
# chmod +x /usr/local/bin/docker-compose
# exit
```

インストールできたらバージョンを確認します。

```
$ docker-compose --version
docker-compose version 1.24.0, build 0aa59064
```

これでインストール完了です。

## macOS

macOSではDockerプロジェクトが配布している「Docker Desktop for Mac」アプリケーションをインストールします。
Mac OS X El Capitan 10.11かそれ以降のmacOSで使用できます。

Docker CEはLinuxカーネルの機能を使っており、コンテナ内で実行されるアプリケーション（例：PostgreSQL）も、macOSのものではなく、Linuxのものとなります。
Docker Desktop for Macではそのために、xhyveという仮想化ハイパーバイザ上でごく軽量なLinux仮想マシンを実行し、その中でコンテナを実行します。

ではインストールしましょう。
Webブラウザで以下のURLを開き「Download from Docker Hub」のボタンを押します。

- https://docs.docker.com/docker-for-mac/install/

Docker Hub内のDocker Desktop for Macのページに移動します。
Docker Hubにログインしている場合には「Get Docker」というボタンが表示されるので、そのボタンを押して`Docker.img`というディスクイメージをダウンロードします。
Docker Hubにログインしていない場合には「Please Login to Download」というボタンが表示されるので、そのボタンを押して指示に従います。

ダウンロードした`Docker.dmg`をダブルクリックしてマウントします。
フォルダが開いたら、そこにある`Docker.app`をアプリケーションフォルダへドラッグします。
`Docker.app`にはDocker CEだけでなくDocker Composeなども含まれていますので、これでインストールは完了です。

インストール結果を確認しましょう。
ラウンチパッドからDockerアプリを起動します。
メニューバーにDockerのクジラのアイコンが表示されますので、クリックしてステータスが「Docker is running」に変わるのを待ちます。
ターミナルを開いて以下のように入力してください。

```
$ docker --version
Docker version 18.09.2, build 6247962

$ docker-compose --version
docker-compose version 1.23.2, build 1110ad01

$ docker run hello-world
...
Hello from Docker!
This message shows that your installation appears to be working correctly.
...
```

これでインストール完了です。

## Windows

WindowsではDockerプロジェクトが配布している「Docker Desktop for Windows」アプリケーションをインストールします。
仮想化ハイパーバイザのHyper-Vを使用しますので、Windows 10（64ビット版）のProfessional以上のエディションが必要です。
もしWindows 10（64ビット版）のHomeエディションをお使いの場合は「Docker Toolbox for Windows」という、少し古いものの仮想化ソフトのVirtualBoxを使用するアプリケーションが使えます。

Docker CEはLinuxカーネルの機能を使っており、コンテナ内で実行されるアプリケーション（例：PostgreSQL）も、Windowsのものではなく、Linuxのものとなります。
Windows版でHyper-VやVirtualBoxを使用しているのはそのためです。ごく軽量なLinux仮想マシンを実行し、その中でコンテナが実行されます。

ではインストールしましょう。
Hyper-VベースのDocker Desktop for Windowsを使用するなら、Webブラウザで以下のURLを開き「Download from Docker Hub」のボタンを押します。

- https://docs.docker.com/docker-for-windows/install/

Docker Hub内のDocker Desktop for Windowsのページに移動します。
Docker Hubにログインしている場合には「Get Docker」というボタンが表示されるので、そのボタンを押してインストーラをダウンロードします。
Docker Hubにログインしていない場合には「Please Login to Download」というボタンが表示されるので、そのボタンを押して指示に従います。

VirtualBoxベースのDocker Toolbox for Windowsを使用するなら、Webブラウザで以下のページを開き、「Get Docker Toolbox for Windows」のボタンを押します。

- https://docs.docker.com/toolbox/overview/

インストーラがダウンロードできたらダブルクリックし、画面の指示に従ってインストールします。
Docker CEだけでなく、Docker Composeなども同時にインストールされます。

インストール結果を確認しましょう。
スタートメニューからDockerアプリを起動します。
タスクトレイにDockerのクジラのアイコンが表示されますので、クリックしてステータスが「Docker is running」に変わるのを待ちます。
起動したらコマンドプロンプトかPowerShellを開いて以下のように入力してください。

```
$ docker --version
Docker version 18.09.2, build 6247962

$ docker-compose --version
docker-compose version 1.23.2, build 1110ad01

PS> docker run hello-world
...
Hello from Docker!
This message shows that your installation appears to be working correctly.
...
```

これでインストール完了です。
