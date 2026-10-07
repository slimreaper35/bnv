# bnv

A beautiful, modern environment variable explorer

[![latest version](https://img.shields.io/crates/v/bnv?color=orange)](https://crates.io/crates/bnv)
[![total downloads](https://img.shields.io/crates/d/bnv?color=green)](https://crates.io/crates/bnv)
[![license](https://img.shields.io/crates/l/bnv?color=yellow)](https://github.com/slimreaper35/bnv/blob/main/LICENSE)
[![documentation](https://img.shields.io/docsrs/bnv?color=blue)](https://docs.rs/bnv/latest/bnv)

![screenshot](assets/screenshot.png)

## Installation

### macOS

```bash
brew install slimreaper35/bnv/bnv
```

### Cargo

```shell
cargo install bnv
```

## Usage

List all environment variables:

```shell
bnv
```

Print a single variable:

```shell
bnv PATH
```

Filter by name with a regex:

```shell
bnv --grep '^AWS_'
```
