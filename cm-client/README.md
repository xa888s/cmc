# \[WIP\] cmc - CrackMes Getter
Just a simple program to download crackmes from crackmes.one

## Install (requires cargo):
```
git clone https://git.cryptid.cc/lost/cmc
cd cmc
cargo install --path .
```

## Example:
This would download and extract the files for [this](https://crackmes.one/crackme/60816eb933c5d42f3852082e) crackme.
```
cmc get 60816eb933c5d42f3852082e
```
This would search for all Linux/Unix crackmes.
```
cmc search --platform linux
```
