#!/bin/bash
echo "-! [0]building..."
cargo build

echo "-! [1]copying..."
# make directory
mkdir -p ~/.nysh
chmod +x ~/.nysh

# move the binary
cp target/debug/nysh ~/.nysh
rm target/debug/nysh
chmod +x ~/.nysh/nysh

echo "-! [2]PATHing..."
# bash PATH
echo "export PATH="~/.nysh"":'"$PATH"' >>~/.profile
source ~/.profile
# fish PATH
mkdir -p ~/.config/fish/conf.d
touch ~/.config/fish/conf.d/nysh.fish
echo "set PATH ~/.nysh" : '"$PATH"' >> ~/.config/fish/conf.d/nysh.fish
source ~/.config/fish/conf.d/nysh.fish

echo
GREEN='\033[0;32m'
GREY='\033[0;36m'
NC='\033[0m' # No Color
echo -e "${GREEN}-! [3]DONE${NC}"
echo