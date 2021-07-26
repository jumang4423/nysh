<!-- DO NOT REMOVE - contributor_list:data:start:["jumang4423", "ranon-rat"]:end -->

<h1 align="center"> <a href="#english">english</a> |<a href="#japanese">日本語</a></h1>

![README LOGO](_img/bk.png)
# nysh | nyu shell

[![Contributor List](https://github.com/jumang4423/nysh/actions/workflows/contributor_list.yml/badge.svg)](https://github.com/jumang4423/nysh/actions/workflows/contributor_list.yml)
[![rust cargo test](https://github.com/jumang4423/nysh/actions/workflows/rust_test.yml/badge.svg?branch=main)](https://github.com/jumang4423/nysh/actions/workflows/rust_test.yml)

a cute shell thingy that written in rust

<!-- prettier-ignore-start -->
<!-- DO NOT REMOVE - contributor_list:start -->
## 👥 Contributors


- **[@jumang4423](https://github.com/jumang4423)**

- **[@ranon-rat](https://github.com/ranon-rat)**

<!-- DO NOT REMOVE - contributor_list:end -->
<!-- prettier-ignore-end -->

<h1 align="left" id="english"> 🇺🇸english<h1>

## dependancies for build

    - rust:nightly 

## installation

run this command below:

```bash
chmod +x scripts/install.sh
./scripts/install.sh
```

default, this script add path **both bash/zsh & fish**

## how to launch

nysh will be installed into ~/.nysh, so path through it or...

- ```nysh``` 

    launch nysh on ur main shell

## blacklisted commands

- ```find```

    to avoid the dream95 injection

- ```emacs```

    i dont use emacs so use vim intead!


## buildin commands

- ```help``` 

    help tool

- ```dream95``` 

    make a secret folder

- ```nywer <filename>``` 

    displays given image on terminal directly

- ```nsd``` 

    simple version of ls command

- ```l``` 

    nsd which just displays files and dircctries

- ```ll``` 

    nsd l which just displays more details

- ```cd``` 

    change directory

- ```..``` 

    equals to ```cd ..```

- ```exit``` 

    exit the shell really safely


## slack notification

if someone entered your secret directory using nysh, nysh notifies to slack using slack bot webhook function.

to do that, export our slack webhook URL like this...

```
DREAM95_SLACK_URL={your slack webhook url}
```


## doesn't work? 
try importing them into each shell settings:
### bash | zsh

``` ~/.profile ```

```bash
export PATH="~/.nysh" : "$PATH" 
```

### fish
    
``` ~/.config/fish/conf.d/nysh.fish```

```bash
set PATH ~/.nysh : "$PATH" 
```

## development with a docker

### 1. launch the docker

```bash
# build go image
sudo docker-compose build
# launch image on background process
sudo docker-compose up -d
# exec go to command using docker envinroment
sudo docker-compose exec nysh cargo run
```

<h1 align="left" id="japanese"> 🇯🇵日本語<h1>

## ビルドのための依存パッケージ

    - rust:nightly 

## インストール方法

以下のコマンドを実行:

```bash
chmod +x scripts/install.sh
./scripts/install.sh
```

~/.nyshに実行可能バイナリが投棄されます

デフォルトでは **bash zsh & fish** にパスが通ります

## 起動方法

nyshは自動的に~/.nyshに実行可能バイナリが投棄されます。よって、その場所をパスに通すもしくは...

- ```nysh``` 

    おてもとのシェルでこれを実行するとnyshが起動します

## blacklisted commands

- ```find```

    to avoid the dream95 injection

- ```emacs```

    i dont use emacs so use vim intead!

## nyshお手製コマンド

- ```help``` 

    ヘルプの表示

- ```dream95``` 

　　秘密のフォルダーを作成

- ```nywer <filename>``` 

    displays given image on terminal directly

- ```nsd``` 

    lsの模擬品

- ```l``` 

    nsdのショートカットで普通に表示する

- ```ll``` 

    nsd lのショートカットで詳細を表示する

- ```cd``` 

    ディレクトリの変更

- ```..``` 

    ```cd ..```

- ```exit``` 

    安全にシェルを終了

## slack通知

誰かがシークレットフォルダーにnyshを使って侵入した場合、webhookを利用して通知をしてくれます！

そうするためには以下のコマンドを実行してください...

```
DREAM95_SLACK_URL={webhookのURL}
```

## 動きません！
あなたのシェルに以下の記述を手動でおねがいします:
### bash | zsh

``` ~/.profile ```

```bash
export PATH="~/.nysh" : "$PATH" 
```

### fish
    
``` ~/.config/fish/conf.d/nysh.fish```

```bash
set PATH ~/.nysh : "$PATH" 
```

## ドッカーを使った開発

### 1. ドッカーを立ち上げます

```bash
# イメージをビルド
sudo docker-compose build
# バックグラウンドでイメージを立ち上げる
sudo docker-compose up -d
# go run *.go をコンテナ内で実行する
sudo docker-compose exec nysh cargo run
```
