# Cite

A tool to generate [Hayagriva](https://github.com/typst/hayagriva) citations from DOI URLs.

## Build instructions

You can build it from source using [Rust](https://rust-lang.org):

```zsh
cargo install --git https://github.com/noClaps/cite
```

## Usage

To use `cite`, you need to pass in a DOI URL to fetch:

```sh
cite 10.5555/12345678
```

This will print the Hayagriva YAML object (with 2 space indentation) for you to paste into your references file.
