<h1 align="center"> <a href="#english">english</a> |<a href="#japanese">日本語</a></h1>

![README LOGO](_img/bk.png)
# nysh | nyu shell

a cute shell that written in RUST

<h1 align="left" id="english"> 🇺🇸english<h1>

## dependancies for build

    - rust:nightly 

## installation

run this command below:

```bash
chmod +x scripts/install.sh
./scripts/install.sh
```

this will install nysh into ~/.nysh

default, this script add path **both bash/zsh & fish**

## usage

- ```nysh``` 

    launch nysh on ur main shell


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

## 使い方

- ```nysh``` 

    おてもとのシェルでこれを実行するとnyshが起動します


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
