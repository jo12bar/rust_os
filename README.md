# `rust_os`

[![Build Code](https://github.com/jo12bar/rust_os/workflows/Build%20Code/badge.svg)](https://github.com/jo12bar/rust_os/actions?query=workflow%3A%22Build+Code%22)

My implementation of [this **amazing** guide](https://os.phil-opp.com/).

## Dependencies

```shell
$ rustup component add rust-src
$ rustup component add llvm-tools-preview
$ cargo install
$ cargo install bootimage
```

##  Building
To build a standalone executable:

```shell
$ cargo build
```

To build a bootable disk image:

```shell
$ cargo bootimage
```

## Booting in QEMU
First, install QEMU. Next, run:

```shell
$ cargo run
```
