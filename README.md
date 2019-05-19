# `rust_os`

[![Build Status](https://travis-ci.com/jo12bar/rust_os.svg?branch=master)](https://travis-ci.com/jo12bar/rust_os)

My implementation of [this **amazing** guide](https://os.phil-opp.com/).

## Dependencies

```shell
$ rustup component add rust-src
$ rustup component add llvm-tools-preview
$ cargo install cargo-xbuild
$ cargo install bootimage --version "^0.7.3"
```

##  Building
To build a standalone executable:

```shell
$ cargo xbuild
```

To build a bootable disk image:

```shell
$ cargo bootimage
```

## Booting in QEMU
First, install QEMU. Next, run:

```shell
$ cargo xrun
```
