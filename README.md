![README LOGO](_img/bk.png)
# nysh | nyu shell

[![Contributor List](https://github.com/jumang4423/nysh/actions/workflows/contributor_list.yml/badge.svg)](https://github.com/jumang4423/nysh/actions/workflows/contributor_list.yml)
[![rust cargo test](https://github.com/jumang4423/nysh/actions/workflows/rust_cargo_test.yml/badge.svg?branch=main)](https://github.com/jumang4423/nysh/actions/workflows/rust_cargo_test.yml)

a cute shell thingy that written in rust

## dependancies for build

    - rust

## installation

run this command below:

```bash
cargo install --git https://github.com/jumang4423/nysh.git
```

## how to launch

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