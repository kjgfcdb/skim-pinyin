# Skim-Pinyin

Fuzzy chinese pinyin searcher written with Rust, powered by `skim`

# Build

```bash
cargo build --release
mv ./targets/release/skim-pinyin /path/to/your/bin
```

# Usage

```bash
# fuzzily search items on current directory
skim-pinyin
# fuzzily search items on ./Downloads
skim-pinyin ./Downloads
# show ignore files
skim-pinyin -s ./Downloads
# allow multiple selections
skim-pinyin -m ./Downloads
# specify dironly flag to filter out files and open with cd
cd $(skim-pinyin -d)
# use vim to open multiple files in current directory
vim $(skim-pinyin -m)
```

# Scripts

Check `scripts` for details

* `fp.fish`: Add `fp` command in fish shell to launch `vim`, `cd` or `rm` with skim-pinyin

# Powered by

* [skim](https://github.com/lotabout/skim)
* [rust-pinyin](https://github.com/mozillazg/rust-pinyin)
