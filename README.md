# emojirs_ls

<img src="assets/demo.png" alt="Demo" width="400px" />

**emojirs_ls** is an alternative to `ls`, written in Rust, fast and minimalist. It displays files in a clear and intuitive way, using emojis to make terminal navigation more visual and enjoyable.

## Installation

Building source code in Linux/BSD systems

```sh
# Cloning Git Repository
git clone https://github.com/jibrildeoculos/emojirs_ls
cd emojirs_ls
# Building and Install
cargo build --release
mv target/release/emojirs_ls /usr/local/bin/
```

## Requirements

Rust toolchain (Cargo & rustc) installed. You can get it from: https://rustup.rs/

## Usage

The usage process is similar to ls

```sh
emojirs_ls documents/ # Show directory
emojirs_ls -a # Show hidden files
```

## License

[MIT License](LICENSE)